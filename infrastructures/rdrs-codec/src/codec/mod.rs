mod video;

use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use ffmpeg_next::{codec, format::{self}, media};
use rdrs_core::{error::Result, infra_trait::Codec, model::{StreamType, VideoStreamInfo}, service::VideoFrameHandler};
use rdrs_tools::tokio_handle;
use tracing::{error, info};
use video::VideoCodec;

pub struct FFmpegCodec {
	stopped:     Arc<AtomicBool>,
	/// NOTE: preserve field, for future change Video info
	video_codec: VideoCodec,
}

/// TODO: split Video and Audio decode to different structs
impl Codec for FFmpegCodec {
	type C = Self;

	fn init() -> Result<()> { Ok(ffmpeg_next::init()?) }

	fn start(source: StreamType, video_frame_handler: Box<dyn VideoFrameHandler>) -> Result<Self::C> {
		info!("Start decoding with source: {source:?}");
		let mut input_context = match source {
			StreamType::File { path } => format::input(&path)?,
			_ => todo!(),
		};
		let video_stream =
			input_context.streams().best(media::Type::Video).ok_or(ffmpeg_next::Error::StreamNotFound)?;

		let (video_sender, video_receiver) = std::sync::mpsc::channel::<codec::packet::Packet>();
		let stopped = Arc::new(AtomicBool::new(false));
		let stopped_current = stopped.clone();

		let video_stream_index = video_stream.index();
		let video_codec =
			VideoCodec::start(video_stream, video_receiver, stopped.clone(), video_frame_handler)?;

		tokio_handle().spawn_blocking(move || {
			for (stream, packet) in input_context.packets() {
				if stopped_current.load(Ordering::Relaxed) {
					break;
				}
				if stream.index() == video_stream_index {
					let _ = video_sender
						.send(packet)
						.inspect_err(|err| error!("Failed to send video packet: {err}"));
				}
			}
		});

		Ok(Self { stopped, video_codec })
	}

	fn change_video_info(&self, new_info: VideoStreamInfo) -> bool {
		self.video_codec.notify_change(new_info)
	}
}

impl Drop for FFmpegCodec {
	fn drop(&mut self) { self.stopped.store(true, Ordering::Relaxed); }
}
