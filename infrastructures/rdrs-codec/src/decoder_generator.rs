use rdrs_domain_player::{service::DecoderGenerator, vo::{StreamSource, VideoInfo}};
use rdrs_tools::error::Result;

use crate::FFmpegDecoder;

#[dep_inj_target::dep_inj_target]
pub struct FFmpegDecoderGenerator;

impl<Deps> DecoderGenerator for FFmpegDecoderGenerator<Deps> {
	type Target = FFmpegDecoder;

	fn init_decoder(&self) -> Result<()> { Ok(ffmpeg_next::init()?) }

	fn generate_decoder(
		&self,
		stream_source: StreamSource,
		render_video_info: VideoInfo,
	) -> Result<Self::Target> {
		Ok(FFmpegDecoder::new(stream_source, render_video_info))
	}
}
