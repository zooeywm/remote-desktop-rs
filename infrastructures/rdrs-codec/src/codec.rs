use ffmpeg_next::{codec::Context as CodecContext, decoder::Video as VideoDecoder, format::{input, Pixel}, media::Type as MediaType, picture, software::scaling::{Context as ScalingContext, Flags as ScalingFlags}, Error as FFmpegError};
use rdrs_core::{error::Result, model::{StreamSource, VideoFrame}, service::Transcoder};
use rdrs_tools::tokio_handle;
use tokio::task::JoinHandle;

use crate::video_frame::FFmpegVideoFrame;

#[derive(dep_inj::DepInj, Default)]
#[target(FFmpegCodec)]
pub struct FFmpegCodecState {
	decoder_join_handles: Vec<JoinHandle<()>>,
}

impl FFmpegCodecState {
	pub fn new() -> Self { Self { decoder_join_handles: vec![] } }
}

impl<Deps> Transcoder for FFmpegCodec<Deps>
where
	Deps: AsMut<FFmpegCodecState>,
{
	fn strat_decode(
		&mut self,
		source: StreamSource,
		on_video_frame: impl Fn(&dyn VideoFrame) -> Result<()> + Send + 'static,
	) -> Result<()> {
		ffmpeg_next::init()?;
		tracing::info!("{source:#?}");
		let mut input_context = match source {
			StreamSource::File { path } => input(&path)?,
			_ => todo!(),
		};

		let video_stream =
			input_context.streams().best(MediaType::Video).ok_or(FFmpegError::StreamNotFound)?;
		let video_stream_index = video_stream.index();
		// let video_stream_time_base = video_stream.time_base();

		let mut video_decoder =
			CodecContext::from_parameters(video_stream.parameters())?.decoder().video()?;

		let mut process_input_context = move || -> Result<()> {
			for (stream, packet) in input_context.packets() {
				if stream.index() == video_stream_index {
					// Send the packet to video decoder
					video_decoder.send_packet(&packet)?;
					for rendered_frame in receive_and_process_decoded(&mut video_decoder)? {
						on_video_frame(&rendered_frame)?;
					}
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
fn receive_and_process_decoded(decoder: &mut VideoDecoder) -> Result<Vec<FFmpegVideoFrame>> {
	let (format, width, height) = (decoder.format(), decoder.width(), decoder.height());
	let mut frame = ffmpeg_next::frame::Video::empty();

	let mut rendered_frames = vec![];
	while decoder.receive_frame(&mut frame).is_ok() {
		let timestamp = frame.timestamp();
		frame.set_pts(timestamp);
		frame.set_kind(picture::Type::None);
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
		let mut rgb_frame = ffmpeg_next::frame::Video::empty();
		scaler.run(&frame, &mut rgb_frame)?;
		rendered_frames.push(FFmpegVideoFrame(rgb_frame));
	}

	Ok(rendered_frames)
}

impl Drop for FFmpegCodecState {
	fn drop(&mut self) {
		for join_handle in &self.decoder_join_handles {
			join_handle.abort()
		}
	}
}
