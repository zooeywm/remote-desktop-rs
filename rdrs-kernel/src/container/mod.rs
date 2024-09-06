mod boilerplate;

use rdrs_codec::FFmpegCodecState;
use rdrs_core::error::Result;

use crate::{config::CommonConfig, telemetry::init_telemetry};

#[derive(derive_more::AsMut)]
pub struct Container {
	#[as_mut]
	pub(crate) transcoder_manager: FFmpegCodecState,
	pub extends:                   Option<config::Value>,
}

impl Container {
	pub fn initialize(CommonConfig { telemetry, extends, .. }: CommonConfig) -> Result<Self> {
		init_telemetry(&telemetry)?;
		std::panic::set_hook(Box::new(|panic_info| {
			tracing::error!("panic: {panic_info}");
		}));

		Ok(Self { transcoder_manager: FFmpegCodecState::new(), extends })
	}
}
