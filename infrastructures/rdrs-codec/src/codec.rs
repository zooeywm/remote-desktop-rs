use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use ffmpeg_next::{codec, decoder, format::{self, Pixel}, frame, media, software::scaling};
use rdrs_core::{error::Result, model::StreamType, service::VideoFrameHandler};
use rdrs_tools::tokio_handle;
use tracing::{error, info};

use crate::{stream_clock::StreamClock, video_frame::FFmpegVideoFrame};

pub(crate) struct FFmpegCodec {
	stop_flag: Arc<AtomicBool>,
}

impl FFmpegCodec {
	pub fn start(
		source: StreamType,
		video_frame_handler: Box<dyn VideoFrameHandler>,
	) -> Result<Self> {
		tracing::info!("Start decoding with source: {source:?}");
		let mut input_context = match source {
			StreamType::File { path } => format::input(&path)?,
			_ => todo!(),
		};

		let video_stream =
			input_context.streams().best(media::Type::Video).ok_or(ffmpeg_next::Error::StreamNotFound)?;
		let video_stream_index = video_stream.index();

		let mut video_decoder =
			codec::Context::from_parameters(video_stream.parameters())?.decoder().video()?;

		let clock = StreamClock::new(&video_stream);

		let (format, width, height) =
			(video_decoder.format(), video_decoder.width(), video_decoder.height());

		let mut video_frame = ffmpeg_next::frame::Video::new(format, width, height);
		let mut rendered_frame = ffmpeg_next::frame::Video::new(Pixel::RGB24, width, height);

		let (video_sender, video_receiver) = std::sync::mpsc::channel::<codec::packet::Packet>();
		let stop_flag = Arc::new(AtomicBool::new(false));
		let stop_flag_video = stop_flag.clone();
		let stop_flag_main = stop_flag.clone();

		tokio_handle().spawn_blocking(move || {
			if let Ok(mut scaler) = scaling::Context::get(
				format,
				width,
				height,
				Pixel::RGB24,
				width,
				height,
				scaling::Flags::FAST_BILINEAR,
			)
			.inspect_err(|err| error!("Failed to get scaler: {err}"))
			{
				while !stop_flag_video.load(Ordering::Relaxed) {
					let Ok(packet) = video_receiver.recv() else {
						info!("Video stream closed");
						break;
					};
					let _ = process_video_packet(
						&mut video_frame,
						&mut rendered_frame,
						&mut video_decoder,
						&clock,
						video_frame_handler.as_ref(),
						&mut scaler,
						&packet,
					)
					.inspect_err(|err| error!("Failed to process_video_packet: {err}"));
				}
			}
		});

		tokio_handle().spawn_blocking(move || {
			for (stream, packet) in input_context.packets() {
				if stop_flag_main.load(Ordering::Relaxed) {
					break;
				}
				if stream.index() == video_stream_index {
					let _ = video_sender
						.send(packet)
						.inspect_err(|err| error!("Failed to send video packet: {err}"));
				}
			}
		});

		Ok(Self { stop_flag })
	}
}

#[inline]
fn process_video_packet(
	video_frame: &mut frame::Video,
	rendered_frame: &mut frame::Video,
	decoder: &mut decoder::Video,
	clock: &StreamClock,
	video_frame_handler: &dyn VideoFrameHandler,
	scaler: &mut scaling::Context,
	packet: &codec::packet::Packet,
) -> Result<()> {
	decoder.send_packet(packet)?;
	while decoder.receive_frame(video_frame).is_ok() {
		if let Some(delay) = clock.convert_pts_to_instant(video_frame.pts()) {
			std::thread::sleep(delay);
		}
		scaler.run(video_frame, rendered_frame)?;
		video_frame_handler.handle_video_frame(&FFmpegVideoFrame(rendered_frame))?;
	}
	Ok(())
}

impl Drop for FFmpegCodec {
	fn drop(&mut self) { self.stop_flag.store(true, Ordering::Relaxed); }
}
