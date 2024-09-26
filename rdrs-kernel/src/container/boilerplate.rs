use rdrs_codec::{FFmpegDecoder, FFmpegDecoderGenerator};
use rdrs_domain_player::{entity::Player, repository::PlayerRepository, service::{DecoderGenerator, PlayerManager, RendererGenerator}, service_impl::PlayerManagerImpl, vo::{StreamSource, VideoInfo}};
use rdrs_gui::{SlintGui, SlintRenderer};
use rdrs_repository::PlayerMemoryRepository;
use rdrs_tools::error::Result;

use crate::container::Container;

impl DecoderGenerator for Container {
	type Target = FFmpegDecoder;

	fn init_decoder(&self) -> Result<()> { FFmpegDecoderGenerator::inj_ref(self).init_decoder() }

	fn generate_decoder(
		&self,
		stream_source: StreamSource,
		render_video_info: VideoInfo,
	) -> Result<Self::Target> {
		FFmpegDecoderGenerator::inj_ref(self).generate_decoder(stream_source, render_video_info)
	}
}

#[cfg(feature = "slint")]
impl RendererGenerator for Container {
	type Target = SlintRenderer;

	fn generate_renderer(&self, renderer_info: VideoInfo) -> Result<Self::Target> {
		SlintGui::inj_ref(self).generate_renderer(renderer_info)
	}
}

impl PlayerRepository for Container {
	fn create(&mut self, player: Player) -> Result<u8> {
		PlayerMemoryRepository::inj_ref_mut(self).create(player)
	}

	fn get_all(&self) -> Vec<&Player> { PlayerMemoryRepository::inj_ref(self).get_all() }

	fn get_mut_by_id(&mut self, id: u8) -> Result<&mut Player> {
		PlayerMemoryRepository::inj_ref_mut(self).get_mut_by_id(id)
	}

	fn get_by_id(&self, id: u8) -> Result<&Player> {
		PlayerMemoryRepository::inj_ref(self).get_by_id(id)
	}
}

impl PlayerManager for Container {
	fn init_decoder(&self) -> Result<()> { PlayerManagerImpl::inj_ref(self).init_decoder() }

	fn create_player(
		&mut self,
		stream_source: StreamSource,
		render_video_info: VideoInfo,
	) -> Result<u8> {
		PlayerManagerImpl::inj_ref_mut(self).create_player(stream_source, render_video_info)
	}

	fn start(&self, id: u8) -> Result<()> { PlayerManagerImpl::inj_ref(self).start(id) }

	fn change_decode_video_info(&self, id: u8, new_info: VideoInfo) -> Result<()> {
		PlayerManagerImpl::inj_ref(self).change_decode_video_info(id, new_info)
	}

	fn change_render_video_info(&self, id: u8, new_info: VideoInfo) -> Result<()> {
		PlayerManagerImpl::inj_ref(self).change_render_video_info(id, new_info)
	}
}
