mod boilerplate;

use anyhow::Result;
use remote_desk_config::AppConfig;

// use remote_desk_core::model::VideoFrame;
use crate::{implements::FFmpegWithRodioStreamDecoderState, FFmpegVideoFrame};

#[derive(derive_more::AsRef)]
pub struct Container {
	#[as_ref]
	stream_decoder: FFmpegWithRodioStreamDecoderState,
}

impl Container {
	pub fn new(
		_config: &AppConfig,
		on_video_frame: impl Fn(&FFmpegVideoFrame) -> Result<()> + Send + 'static,
	) -> Self {
		let stream_decoder = FFmpegWithRodioStreamDecoderState::new(on_video_frame);
		Self { stream_decoder }
	}
}
