mod boilerplate;

use remote_desk_codec::FFmpegCodecState;

use crate::config::CommonConfig;

#[derive(derive_more::AsMut)]
pub struct Container {
	#[as_mut]
	transcoder_manager: FFmpegCodecState,
}

impl Container {
	pub fn new(_config: &CommonConfig) -> Self {
		Self { transcoder_manager: FFmpegCodecState::new() }
	}
}
