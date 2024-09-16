use rdrs_tools::error::Result;

use super::Renderer;
use crate::vo::{StreamSource, VideoInfo};

/// Decoder generator
pub trait DecoderGenerator {
	type Target: Decoder;

	/// Init the decoder environment, this should only be called once.
	fn init_decoder(&self) -> Result<()>;

	/// Generate a new decoder.
	fn generate_decoder(
		&self,
		stream_source: StreamSource,
		render_video_info: VideoInfo,
	) -> Result<Self::Target>;
}

/// Decoder
pub trait Decoder: Send + 'static {
	type Renderer: Renderer;

	/// Start decoding and rendering and return initial VideoInfo.
	fn start(&mut self, renderer: Self::Renderer) -> Result<VideoInfo>;
}
