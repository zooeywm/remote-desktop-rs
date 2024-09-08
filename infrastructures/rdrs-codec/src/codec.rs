use ffmpeg_next::{codec::Context as CodecContext, decoder::Video as VideoDecoder, format::{input, Pixel}, media::Type as MediaType, software::scaling::{Context as ScalingContext, Flags as ScalingFlags}, Error as FFmpegError};
use rdrs_core::{error::Result, model::StreamSource, service::{Codec, VideoFrameHandler}};
use rdrs_tools::tokio_handle;
use tokio::task::JoinHandle;

use crate::{stream_clock::StreamClock, video_frame::FFmpegVideoFrame};

#[derive(dep_inj::DepInj, Default)]
#[target(FFmpegCodec)]
pub struct FFmpegCodecState {
	decoder_join_handles: Vec<JoinHandle<()>>,
}

impl FFmpegCodecState {
	pub fn new() -> Self { Self { decoder_join_handles: vec![] } }
}

impl<Deps> Codec for FFmpegCodec<Deps>
where
	Deps: AsMut<FFmpegCodecState>,
{
	fn strat_decode(
		&mut self,
		source: StreamSource,
		video_frame_handler: Box<dyn VideoFrameHandler>,
	) -> Result<()> {
		ffmpeg_next::init()?;
		tracing::info!("Start decoding with source: {source:?}");
		let mut input_context = match source {
			StreamSource::File { path } => input(&path)?,
			_ => todo!(),
		};

		let video_stream =
			input_context.streams().best(MediaType::Video).ok_or(FFmpegError::StreamNotFound)?;
		let video_stream_index = video_stream.index();

		let mut video_decoder =
			CodecContext::from_parameters(video_stream.parameters())?.decoder().video()?;

		let clock = StreamClock::new(&video_stream);

		let (format, width, height) =
			(video_decoder.format(), video_decoder.width(), video_decoder.height());

		let mut video_frame = ffmpeg_next::frame::Video::new(format, width, height);
		let mut rendered_frame = ffmpeg_next::frame::Video::new(Pixel::RGB24, width, height);

		let mut process_input_context = move || -> Result<()> {
			// Render with software scale
			let mut scaler = ScalingContext::get(
				format,
				width,
				height,
				Pixel::RGB24,
				width,
				height,
				ScalingFlags::FAST_BILINEAR,
			)?;
			for (stream, packet) in input_context.packets() {
				if stream.index() == video_stream_index {
					// Send the packet to video decoder
					video_decoder.send_packet(&packet)?;
					video_receive_and_process_decode(
						&mut video_frame,
						&mut rendered_frame,
						&mut video_decoder,
						&clock,
						video_frame_handler.as_ref(),
						&mut scaler,
					)?;
				}
			}
			video_decoder.send_eof()?;
			Ok(())
		};

		let join_handle = tokio_handle().spawn_blocking(move || {
			if let Err(err) = process_input_context() {
				tracing::error!("Failed to process video input context: {err}");
			}
		});

		self.deps.as_mut().decoder_join_handles.push(join_handle);

		Ok(())
	}
}

#[inline]
fn video_receive_and_process_decode(
	video_frame: &mut ffmpeg_next::frame::Video,
	rendered_frame: &mut ffmpeg_next::frame::Video,
	decoder: &mut VideoDecoder,
	clock: &StreamClock,
	video_frame_handler: &dyn VideoFrameHandler,
	scaler: &mut ScalingContext,
) -> Result<()> {
	while decoder.receive_frame(video_frame).is_ok() {
		if let Some(delay) = clock.convert_pts_to_instant(video_frame.pts()) {
			std::thread::sleep(delay);
		}
		scaler.run(video_frame, rendered_frame)?;
		video_frame_handler.handle_video_frame(&FFmpegVideoFrame(rendered_frame))?;
	}
	Ok(())
}

impl Drop for FFmpegCodecState {
	fn drop(&mut self) {
		for join_handle in &self.decoder_join_handles {
			join_handle.abort()
		}
	}
}
