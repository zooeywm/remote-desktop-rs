use dep_inj_target::dep_inj_target;
use ffmpeg_next::{format::{input, Pixel}, media::Type, software::scaling::Flags};
use remote_desk_core::{error::Result, model::StreamSource, service::StreamDecoder};
use remote_desk_tools::tokio_handle;

use crate::{CodecContext, FFmpegError, FFmpegVideoFrame, ScalingContext, VideoDecoder, VideoFrame};

#[dep_inj_target]
pub struct FFmpegWithRodioStreamDecoder;

impl<Deps> StreamDecoder for FFmpegWithRodioStreamDecoder<Deps> {
	fn decode_stream(
		&self,
		source: StreamSource,
		on_video_frame: impl Fn(&dyn VideoFrame) -> Result<()> + Send + 'static,
	) -> Result<()> {
		ffmpeg_next::init()?;
		let mut input = match source {
			StreamSource::File { path } => input(&path)?,
			_ => todo!(),
		};
		let video_stream =
			input.streams().best(Type::Video).ok_or(FFmpegError::StreamNotFound).unwrap();
		let video_stream_index = video_stream.index();

		let mut video_decoder =
			CodecContext::from_parameters(video_stream.parameters())?.decoder().video()?;

		tokio_handle().spawn_blocking(move || {
			for (stream, packet) in input.packets() {
				if stream.index() == video_stream_index {
					// Send the packet to video decoder
					video_decoder.send_packet(&packet).unwrap();
					let rendered_frame = decode_video_packet(&mut video_decoder).unwrap();
					on_video_frame(&rendered_frame).unwrap();
				}
			}
			video_decoder.send_eof().unwrap()
		});
		Ok(())
	}
}

#[inline]
fn decode_video_packet(decoder: &mut VideoDecoder) -> Result<FFmpegVideoFrame> {
	let (format, width, height) = (decoder.format(), decoder.width(), decoder.height());

	let mut decoded = ffmpeg_next::frame::Video::empty();

	decoder.receive_frame(&mut decoded)?;

	// Render with software scale
	let mut scaler =
		ScalingContext::get(format, width, height, Pixel::RGB24, width, height, Flags::FAST_BILINEAR)?;
	let mut rgb_frame = FFmpegVideoFrame::empty();
	scaler.run(&decoded, &mut rgb_frame)?;

	Ok(rgb_frame)
}
