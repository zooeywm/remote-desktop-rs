use std::time::Instant;

use bytemuck::Pod;
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SizedSample, SupportedStreamConfig};
use ffmpeg_next::{codec, decoder, format::{self, sample}, frame, media, software::{resampling, scaling}, util::channel_layout::ChannelLayout};
use rdrs_domain_player::{service::{Decoder, Renderer}, vo::{Rational, StreamClock, StreamSource, VideoInfo}};
use rdrs_gui::SlintRenderer;
use rdrs_tools::{error::Result, tokio_handle};
use ringbuf::{traits::{Consumer, Observer, Producer, Split}, HeapRb};
use tracing::{error, info, warn};

use crate::pixel_transform::{ffmpeg_pixel_to_pixel, pixel_to_ffmpeg_pixel};

pub struct FFmpegDecoder {
	stream_source:     StreamSource,
	render_video_info: VideoInfo,
}

impl Decoder for FFmpegDecoder {
	type Renderer = SlintRenderer;

	fn start(&mut self, renderer: Self::Renderer) -> Result<VideoInfo> {
		let mut input_context = match self.stream_source {
			StreamSource::File { ref path } => format::input(path)?,
		};
		let video_stream =
			input_context.streams().best(media::Type::Video).ok_or(ffmpeg_next::Error::StreamNotFound)?;
		let audio_stream =
			input_context.streams().best(media::Type::Audio).ok_or(ffmpeg_next::Error::StreamNotFound)?;
		let video_stream_index = video_stream.index();
		let audio_stream_index = audio_stream.index();
		let mut video_decoder =
			codec::Context::from_parameters(video_stream.parameters())?.decoder().video()?;
		let audio_decoder =
			codec::Context::from_parameters(audio_stream.parameters())?.decoder().audio()?;
		let (format, width, height) =
			(video_decoder.format(), video_decoder.width(), video_decoder.height());
		let VideoInfo { width: target_width, height: target_height, format: target_format } =
			self.render_video_info;
		let decode_video_info = VideoInfo::new(width, height, ffmpeg_pixel_to_pixel(format));
		let target_format = pixel_to_ffmpeg_pixel(target_format);
		let video_rational = video_stream.time_base();
		let audio_rational = audio_stream.time_base();
		let video_stream_clock =
			StreamClock::new(Rational::new(video_rational.numerator(), video_rational.denominator()));
		let audio_stream_clock =
			StreamClock::new(Rational::new(audio_rational.numerator(), audio_rational.denominator()));
		tracing::info!("{video_stream_clock:?}\n{audio_stream_clock:?}");

		let (video_sender, video_receiver) = std::sync::mpsc::channel::<codec::packet::Packet>();
		let (audio_sender, audio_receiver) = std::sync::mpsc::channel::<codec::packet::Packet>();

		// Video decode thread.
		tokio_handle().spawn_blocking(move || {
			if let Err(err) = move || -> Result<()> {
				let mut scaler = scaling::Context::get(
					format,
					width,
					height,
					target_format,
					target_width,
					target_height,
					scaling::Flags::FAST_BILINEAR,
				)?;

				let mut video_frame_buffer = frame::Video::new(format, width, height);
				let mut scale_frame_buffer = frame::Video::new(target_format, target_width, target_height);

				loop {
					let Ok(ref packet) = video_receiver.recv() else {
						info!("Video stream closed");
						break Ok(());
					};
					video_decoder.send_packet(packet)?;
					while video_decoder.receive_frame(&mut video_frame_buffer).is_ok() {
						let instant = Instant::now();
						scaler.run(&video_frame_buffer, &mut scale_frame_buffer)?;
						if let Some(delay) = video_stream_clock.convert_pts_to_instant(video_frame_buffer.pts())
						{
							info!("video delay: {delay:?}");
							std::thread::sleep(delay);
						}
						renderer.render(scale_frame_buffer.data(0))?;
						let elapsed = instant.elapsed();
						info!("elapsed: {}ms", elapsed.as_millis());
					}
				}
			}() {
				error!("{err}");
			}
		});

		let host = cpal::default_host();
		let device = host.default_output_device().ok_or(cpal::BuildStreamError::DeviceNotAvailable)?;
		let config = device.default_output_config()?;
		let sample_format = config.sample_format();

		// Audio decode thread
		#[rustfmt::skip]
		tokio_handle().spawn_blocking(move || {
            if let Err(err) = match sample_format {
                cpal::SampleFormat::I16 => start_audio_decode::<i16>(sample::Sample::I16(sample::Type::Packed), device, config, audio_decoder, audio_receiver, audio_stream_clock),
                cpal::SampleFormat::I32 => start_audio_decode::<i32>(sample::Sample::I32(sample::Type::Packed), device, config, audio_decoder, audio_receiver, audio_stream_clock),
                cpal::SampleFormat::I64 => start_audio_decode::<i64>(sample::Sample::I64(sample::Type::Packed), device, config, audio_decoder, audio_receiver, audio_stream_clock),
                cpal::SampleFormat::U8  => start_audio_decode::<u8>( sample::Sample::U8( sample::Type::Packed), device, config, audio_decoder, audio_receiver, audio_stream_clock),
                cpal::SampleFormat::F32 => start_audio_decode::<f32>(sample::Sample::F32(sample::Type::Packed), device, config, audio_decoder, audio_receiver, audio_stream_clock),
                cpal::SampleFormat::F64 => start_audio_decode::<f64>(sample::Sample::F64(sample::Type::Packed), device, config, audio_decoder, audio_receiver, audio_stream_clock),
                format => {
                    error!("Unsupported format: {format:?}");
                    Err(cpal::BuildStreamError::StreamConfigNotSupported.into())
                }
            } {
                error!("{err}");
            }
		});

		// Decoder main thread.
		tokio_handle().spawn_blocking(move || {
			for (stream, packet) in input_context.packets() {
				if stream.index() == video_stream_index {
					if let Err(err) = video_sender.send(packet) {
						error!("Failed to send video packet: {err}")
					};
				} else if stream.index() == audio_stream_index {
					if let Err(err) = audio_sender.send(packet) {
						error!("Failed to send audio packet: {err}")
					};
				}
			}
		});

		Ok(decode_video_info)
	}
}

fn start_audio_decode<T>(
	sample_format: format::Sample,
	device: cpal::Device,
	config: SupportedStreamConfig,
	mut audio_decoder: decoder::Audio,
	audio_receiver: std::sync::mpsc::Receiver<codec::packet::Packet>,
	stream_clock: StreamClock,
) -> Result<()>
where
	T: Send + Pod + SizedSample + 'static,
{
	tracing::info!("{:?}", config.channels());
	let output_channel_layout = match config.channels() {
		1 => ChannelLayout::MONO,
		2 => ChannelLayout::STEREO,
		_ => {
			warn!("Audio deivce more than 2 is not implemented, fallback to STEREO ");
			ChannelLayout::STEREO
		}
	};
	let buffer = HeapRb::new(4096);
	let (mut sample_producer, mut sample_consumer) = buffer.split();
	let cpal_stream = device.build_output_stream(
		&config.config(),
		move |data, _| {
			let filled = sample_consumer.pop_slice(data);
			data[filled..].fill(T::EQUILIBRIUM);
		},
		move |err| error!("Failed feeding audio stream to cpal: {err}"),
		None,
	)?;
	cpal_stream.play()?;
	let (format, layout) = (audio_decoder.format(), audio_decoder.channel_layout());
	let mut resampler = resampling::Context::get(
		format,
		layout,
		audio_decoder.rate(),
		sample_format,
		output_channel_layout,
		config.sample_rate().0,
	)?;

	let mut audio_frame_buffer = frame::Audio::new(format, 0, layout);
	let mut resample_frame_buffer = frame::Audio::new(sample_format, 0, output_channel_layout);

	loop {
		let Ok(ref packet) = audio_receiver.recv() else {
			info!("Audio stream closed");
			break Ok(());
		};
		audio_decoder.send_packet(packet)?;

		while audio_decoder.receive_frame(&mut audio_frame_buffer).is_ok() {
			resampler.run(&audio_frame_buffer, &mut resample_frame_buffer)?;
			let expected_bytes = resample_frame_buffer.samples()
				* resample_frame_buffer.channels() as usize
				* size_of::<T>();
			let cpal_sample_data: &[T] =
				bytemuck::cast_slice(&resample_frame_buffer.data(0)[..expected_bytes]);
			while sample_producer.vacant_len() < cpal_sample_data.len() {}
			if let Some(delay) = stream_clock.convert_pts_to_instant(audio_frame_buffer.pts()) {
				info!("audio delay: {delay:?}");
				std::thread::sleep(delay);
			}
			sample_producer.push_slice(cpal_sample_data);
		}
	}
}

impl FFmpegDecoder {
	pub fn new(stream_source: StreamSource, render_video_info: VideoInfo) -> Self {
		Self { stream_source, render_video_info }
	}
}
