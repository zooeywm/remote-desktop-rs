use rdrs_domain_player::{entity::Player, repository::PlayerRepository, service::PlayerManager, vo::{StreamSource, VideoInfo}};
use rdrs_tools::error::Result;

use crate::container::Container;

impl Container {
	/// Start decoding, and rendering.
	pub fn start_decoding_streaming(
		&mut self,
		stream_source: StreamSource,
		render_video_info: VideoInfo,
	) -> Result<()> {
		let id = self.create(Player::new(stream_source, render_video_info))?;
		self.start(id)?;
		Ok(())
		// self.start_decode(stream_source)
	}

	// pub fn stop_codec_by_id(&self, id: u8) -> Result<()> { self.close_by_id(id) }
	//
	// pub fn stop_all_codecs(&self) -> Result<()> { self.close_all() }

	pub fn start_gui(&self) -> Result<()> { self.run_gui() }
}
