use ffmpeg_next::{codec, format, frame, media, software::scaling};
use rdrs_domain_player::{service::{Decoder, Renderer}, vo::{Rational, StreamClock, StreamSource, VideoInfo}};
use rdrs_gui::SlintRenderer;
use rdrs_tools::{error::Result, tokio_handle};
use tracing::{error, info};

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
		let video_stream_index = video_stream.index();
		let mut video_decoder =
			codec::Context::from_parameters(video_stream.parameters())?.decoder().video()?;
		let (format, width, height) =
			(video_decoder.format(), video_decoder.width(), video_decoder.height());
		let VideoInfo { width: target_width, height: target_height, format: target_format } =
			self.render_video_info;
		let decode_video_info = VideoInfo::new(width, height, ffmpeg_pixel_to_pixel(format));
		let target_format = pixel_to_ffmpeg_pixel(target_format);
		let ffmpeg_rational = video_stream.time_base();
		let stream_clock =
			StreamClock::new(Rational::new(ffmpeg_rational.numerator(), ffmpeg_rational.denominator()));

		let (video_sender, video_receiver) = std::sync::mpsc::channel::<codec::packet::Packet>();

		// Video Decode thread.
		tokio_handle().spawn_blocking(move || {
			let mut scaler = match scaling::Context::get(
				format,
				width,
				height,
				target_format,
				target_width,
				target_height,
				scaling::Flags::FAST_BILINEAR,
			) {
				Ok(v) => v,
				Err(err) => {
					error!("Failed to get scaler: {err}");
					return;
				}
			};

			let mut video_frame_buffer = frame::Video::new(format, width, height);
			let mut scale_frame_buffer = frame::Video::new(target_format, target_width, target_height);

			loop {
				let Ok(ref packet) = video_receiver.recv() else {
					info!("Video stream closed");
					break;
				};
				if let Err(err) = video_decoder.send_packet(packet) {
					error!("Failed send packet: {err}")
				}
				while video_decoder.receive_frame(&mut video_frame_buffer).is_ok() {
					if let Some(delay) = stream_clock.convert_pts_to_instant(video_frame_buffer.pts()) {
						std::thread::sleep(delay);
					}
					if let Err(err) = scaler.run(&video_frame_buffer, &mut scale_frame_buffer) {
						error!("Failed to scale: {err}");
					};
					if let Err(err) = renderer.render(scale_frame_buffer.data(video_stream_index)) {
						error!("Failed render: {err}")
					}
				}
			}
		});

		// Decoder main thread.
		tokio_handle().spawn_blocking(move || {
			for (stream, packet) in input_context.packets() {
				if stream.index() == video_stream_index {
					let _ = video_sender
						.send(packet)
						.inspect_err(|err| error!("Failed to send video packet: {err}"));
				}
			}
		});

		Ok(decode_video_info)
	}
}

impl FFmpegDecoder {
	pub fn new(stream_source: StreamSource, render_video_info: VideoInfo) -> Self {
		Self { stream_source, render_video_info }
	}
}
