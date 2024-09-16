use rdrs_tools::error::Result;

use crate::vo::{StreamSource, VideoInfo};

/// Service to manage video decode and render.
pub trait PlayerManager {
	fn init_decoder(&self) -> Result<()>;

	/// Create a player to control decode and render.
	fn create_player(
		&mut self,
		stream_source: StreamSource,
		render_video_info: VideoInfo,
	) -> Result<u8>;

	/// Start a player by id, it will start decoding and render every decoded
	/// frame.
	fn start(&self, id: u8) -> Result<()>;

	/// Change decoder video info
	fn change_decode_video_info(&self, id: u8, new_info: VideoInfo) -> Result<()>;

	/// Change renderer video info
	fn change_render_video_info(&self, id: u8, new_info: VideoInfo) -> Result<()>;
}
