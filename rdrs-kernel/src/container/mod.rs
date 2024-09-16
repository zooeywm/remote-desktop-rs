mod boilerplate;

use rdrs_codec::FFmpegDecoder;
use rdrs_domain_player::service_impl::PlayerManagerState;
use rdrs_gui::{SlintGuiState, SlintRenderer};
use rdrs_repository::PlayerMemoryRepositoryState;
use rdrs_tools::error::Result;

use crate::{config::CommonConfig, telemetry::init_telemetry};

#[derive(derive_more::AsRef, derive_more::AsMut)]
pub struct Container {
	#[as_ref]
	#[cfg(feature = "slint")]
	pub(crate) gui:               SlintGuiState,
	pub extends:                  Option<config::Value>,
	#[as_ref]
	#[as_mut]
	pub(crate) player_repository: PlayerMemoryRepositoryState,
	#[as_ref]
	#[as_mut]
	pub(crate) player_manager:    PlayerManagerState<FFmpegDecoder, SlintRenderer>,
}

impl Container {
	pub fn initialize(CommonConfig { telemetry, extends, .. }: CommonConfig) -> Result<Self> {
		init_telemetry(&telemetry)?;
		std::panic::set_hook(Box::new(|panic_info| {
			tracing::error!("panic: {panic_info}");
		}));

		#[cfg(feature = "slint")]
		let gui = SlintGuiState::new()?;

		let player_repository = PlayerMemoryRepositoryState::new();
		let player_manager = PlayerManagerState::new();

		Ok(Self { gui, extends, player_repository, player_manager })
	}

	pub fn run_gui(&self) -> Result<()> {
		#[cfg(feature = "slint")]
		self.gui.run()
	}
}
