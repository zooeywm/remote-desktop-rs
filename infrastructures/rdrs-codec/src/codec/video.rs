use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use ffmpeg_next::{codec, decoder, format::{self, Pixel}, frame, software::scaling};
use rdrs_core::{error::Result, model::vo::VideoStreamInfo, service::VideoFrameHandler};
use rdrs_tools::tokio_handle;
use tracing::{error, info};

use crate::{pixel::PixelWrapper, stream_clock::StreamClock, video_frame::FFmpegVideoFrame};

pub(super) struct VideoCodec {
	info_sender: tokio::sync::watch::Sender<VideoStreamInfo>,
}

impl VideoCodec {
	pub fn start(
		stream: format::stream::Stream,
		video_receiver: std::sync::mpsc::Receiver<codec::packet::Packet>,
		stopped: Arc<AtomicBool>,
		video_frame_handler: Box<dyn VideoFrameHandler>,
	) -> Result<Self> {
		let mut video_decoder =
			codec::Context::from_parameters(stream.parameters())?.decoder().video()?;
		let clock = StreamClock::new(&stream);
		let stream_info = VideoStreamInfo {
			format: PixelWrapper(video_decoder.format()).into(),
			width:  video_decoder.width(),
			height: video_decoder.height(),
		};
		let (info_sender, mut info_receiver) = tokio::sync::watch::channel(stream_info);
		tokio_handle().spawn_blocking(move || {
			// This loop is entered everytime the VideoStreamInfo is changed, if the
			// VideoStreamInfo has never changed, this will only enter once.
			let mut closed = false;
			while !closed {
				let VideoStreamInfo { format, width, height } = *info_receiver.borrow_and_update();

				let format = PixelWrapper::from(format).0;

				let mut video_frame = ffmpeg_next::frame::Video::new(format, width, height);
				// NOTE: Hard code the render format currently, in the future, the render logic
				// will be split into other component
				let mut rendered_frame = ffmpeg_next::frame::Video::new(Pixel::RGB24, width, height);

				let mut scaler = match scaling::Context::get(
					format,
					width,
					height,
					// NOTE: Hard code currently
					Pixel::RGB24,
					width,
					height,
					scaling::Flags::FAST_BILINEAR,
				) {
					Ok(v) => v,
					Err(err) => {
						error!("Failed to get scaler: {err}");
						return;
					}
				};

				closed = loop {
					if stopped.load(Ordering::Relaxed) {
						break true;
					}
					if let Ok(changed) = info_receiver.has_changed().inspect_err(|err| {
						error!("info receiver has closed, no more update will receive: {err}")
					}) {
						// If the VideoStreamInfo has changed, break the packet receiving loop,
						// with closed = false, to process with the new VideoStreamInfo
						if changed {
							break false;
						}
					};
					let Ok(packet) = video_receiver.recv() else {
						info!("Video stream closed");
						break true;
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
				};
			}
		});
		Ok(Self { info_sender })
	}

	/// Notify the VideoFrameInfo is changing, retrun true if change, false if
	/// nothing to change
	pub fn notify_change(&self, new_info: VideoStreamInfo) -> bool {
		self.info_sender.send_if_modified(|info| {
			if info.ne(&&new_info) {
				*info = new_info;
				true
			} else {
				false
			}
		})
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
