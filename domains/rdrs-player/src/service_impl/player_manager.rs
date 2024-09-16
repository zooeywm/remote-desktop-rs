use std::marker::PhantomData;

use rdrs_tools::error::Result;

use crate::{repository::PlayerRepository, service::{Decoder, DecoderGenerator, PlayerManager, Renderer, RendererGenerator}, vo::{StreamSource, VideoInfo}, Player};

#[derive(dep_inj::DepInj)]
#[target(PlayerManagerImpl)]
pub struct PlayerManagerState<Dec, Ren> {
	decoder:  PhantomData<Dec>,
	renderer: PhantomData<Ren>,
}

impl<Dec, Ren> PlayerManagerState<Dec, Ren> {
	pub fn new() -> Self { Self { decoder: PhantomData, renderer: PhantomData } }
}

impl<Dec, Ren> Default for PlayerManagerState<Dec, Ren> {
	fn default() -> Self { Self::new() }
}

impl<Dec, Ren, Deps> PlayerManager for PlayerManagerImpl<Dec, Ren, Deps>
where
	Deps: PlayerRepository + DecoderGenerator<Target = Dec> + RendererGenerator<Target = Ren>,
	Dec: Decoder<Renderer = Ren>,
	Ren: Renderer,
{
	fn init_decoder(&self) -> Result<()> { self.prj_ref().init_decoder() }

	fn create_player(
		&mut self,
		stream_source: StreamSource,
		render_video_info: VideoInfo,
	) -> Result<u8> {
		self.prj_ref_mut().create(Player::new(stream_source, render_video_info))
	}

	fn start(&self, id: u8) -> Result<()> {
		let deps = self.prj_ref();
		let player = deps.get_by_id(id)?;

		let mut decoder =
			deps.generate_decoder(player.stream_source.clone(), player.render_video_info)?;
		let renderer = deps.generate_renderer(player.render_video_info)?;

		let _decode_video_info = decoder.start(renderer)?;
		Ok(())
	}

	fn change_decode_video_info(&self, _id: u8, _new_info: VideoInfo) -> Result<()> { todo!() }

	fn change_render_video_info(&self, _id: u8, _new_info: VideoInfo) -> Result<()> { todo!() }
}
