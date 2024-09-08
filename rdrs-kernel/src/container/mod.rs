mod boilerplate;

use rdrs_codec::FFmpegCodecManagerState;
use rdrs_core::error::Result;
use rdrs_gui::SlintGuiState;

use crate::{config::CommonConfig, telemetry::init_telemetry};

#[derive(derive_more::AsRef)]
pub struct Container {
	#[as_ref]
	pub(crate) codec: FFmpegCodecManagerState,
	#[as_ref]
	#[cfg(feature = "slint")]
	pub(crate) gui:   SlintGuiState,
	pub extends:      Option<config::Value>,
}

impl Container {
	pub fn initialize(CommonConfig { telemetry, extends, .. }: CommonConfig) -> Result<Self> {
		init_telemetry(&telemetry)?;
		std::panic::set_hook(Box::new(|panic_info| {
			tracing::error!("panic: {panic_info}");
		}));

		#[cfg(feature = "slint")]
		let gui = SlintGuiState::new()?;

		Ok(Self { codec: FFmpegCodecManagerState::new()?, gui, extends })
	}
}
