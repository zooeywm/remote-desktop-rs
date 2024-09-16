use rdrs_tools::error::Result;

use crate::vo::VideoInfo;

/// Video renderer generator.
pub trait RendererGenerator {
	type Target: Renderer;

	/// Generate a new renderer.
	fn generate_renderer(&self, renderer_info: VideoInfo) -> Result<Self::Target>;
}

/// Decoded frame renderer.
pub trait Renderer {
	/// Render the decoded video frame.
	fn render(&self, decode_buffer: &[u8]) -> Result<()>;
}
