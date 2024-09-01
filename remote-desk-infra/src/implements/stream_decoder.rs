use std::{cell::RefCell, sync::OnceLock};

use anyhow::{anyhow, Context, Result};
use ffmpeg_next::{format::{input, Pixel}, media::Type, software::scaling::Flags, Packet};
use remote_desk_core::{model::StreamSource, service::StreamDecoder};
use remote_desk_tools::tokio_handle;
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use crate::{CodecContext, FFmpegError, FFmpegVideoFrame, OnVideoFrame, ScalingContext, VideoDecoder};

#[derive(dep_inj::DepInj)]
#[target(FFmpegWithRodioStreamDecoder)]
pub struct FFmpegWithRodioStreamDecoderState {
	display_sender:        OnceLock<Sender<FFmpegVideoFrame>>,
	display_thread_handle: OnceLock<JoinHandle<()>>,
	on_video_frame:        RefCell<Option<Box<OnVideoFrame>>>,
}

impl<Deps> StreamDecoder for FFmpegWithRodioStreamDecoder<Deps>
where
	Deps: AsRef<FFmpegWithRodioStreamDecoderState> + Sized,
{
	fn init(&self) -> Result<()> {
		ffmpeg_next::init().context("FFmpeg init")?;
		let (display_sender, mut display_receiver) = tokio::sync::mpsc::channel::<FFmpegVideoFrame>(60);
		self.display_sender.set(display_sender).map_err(|_| anyhow!("Set display sender")).unwrap();

		let on_video_frame = self.on_video_frame.borrow_mut().take().unwrap();
		let handle = tokio_handle().spawn(async move {
			let mut x = 0;
			while let Some(frame) = display_receiver.recv().await {
				on_video_frame(&frame).context("On video frame").unwrap();
				println!("x: {x}");
				x += 1;
			}
			println!("Display Channel Closed")
		});

		self.display_thread_handle.set(handle).map_err(|_| anyhow!("Set display JoinHandle"))?;

		Ok(())
	}

	fn handle_stream(&self, source: StreamSource) -> Result<()> {
		let mut input = match source {
			StreamSource::File { path } => input(&path)?,
			_ => todo!(),
		};
		let video_stream =
			input.streams().best(Type::Video).context(FFmpegError::StreamNotFound).context("video")?;
		let video_stream_index = video_stream.index();
		let mut video_decoder =
			CodecContext::from_parameters(video_stream.parameters())?.decoder().video()?;

		let display_sender = self.display_sender.get().context("Get display_sender")?.clone();

		tokio_handle().spawn_blocking(move || {
			for (stream, packet) in input.packets() {
				// let time_base_seconds = stream.time_base();
				// let time_base_seconds =
				// 	time_base_seconds.numerator() as f64 / time_base_seconds.denominator() as
				// f64;

				if stream.index() == video_stream_index {
					handle_video_packet(
						// time_base_seconds,
						display_sender.clone(),
						&mut video_decoder,
						packet,
					)
					.unwrap();
				}
			}
			video_decoder.send_eof().context("Flush video_decoder")
		});
		Ok(())
	}
}

impl FFmpegWithRodioStreamDecoderState {
	pub fn new(on_video_frame: impl Fn(&FFmpegVideoFrame) -> Result<()> + Send + 'static) -> Self {
		Self {
			display_sender:        OnceLock::new(),
			display_thread_handle: OnceLock::new(),
			on_video_frame:        RefCell::new(Some(Box::new(on_video_frame))),
		}
	}
}

impl Drop for FFmpegWithRodioStreamDecoderState {
	fn drop(&mut self) {
		let _ = &self.display_sender.get().context("Drop display_sender").unwrap();
		let _ = self.display_thread_handle.get().context("Drop display handle").unwrap();
	}
}

fn handle_video_packet(
	// time_base_seconds: f64,
	display_sender: Sender<FFmpegVideoFrame>,
	decoder: &mut VideoDecoder,
	packet: Packet,
) -> Result<()> {
	let mut decoded = FFmpegVideoFrame::empty();

	let format = decoder.format();
	let width = decoder.width();
	let height = decoder.height();

	decoder.send_packet(&packet)?;

	decoder.receive_frame(&mut decoded).context("Video receive frame")?;
	// let pts = decoded.pts().context("No pts")?;

	// let pts_since_start = Duration::from_secs_f64(pts as f64 *
	// time_base_seconds);

	// let absolute_pts = start_time.checked_add(pts_since_start).context("Check add
	// pts")?;

	// std::thread::sleep(pts_since_start);

	// tokio_handle().spawn_blocking(move || {
	let mut scaler =
		ScalingContext::get(format, width, height, Pixel::RGB24, width, height, Flags::BILINEAR)
			.context("Get scaler")
			.unwrap();
	let mut rgb_frame = FFmpegVideoFrame::empty();
	scaler.run(&decoded, &mut rgb_frame).context("Scaling").unwrap();
	display_sender.blocking_send(rgb_frame).context("Failed send display").unwrap();
	// std::thread::sleep(Duration::from_millis(33));
	// });

	Ok(())
}
