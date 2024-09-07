use rdrs_codec::FFmpegCodec;
use rdrs_core::{error::Result, model::StreamSource, service::{Codec, Gui, VideoFrameHandler}};
use rdrs_gui::SlintGui;

use crate::container::Container;

impl Codec for Container {
	fn strat_decode(
		&mut self,
		source: StreamSource,
		video_frame_handler: Box<dyn VideoFrameHandler>,
	) -> Result<()> {
		FFmpegCodec::inj_ref_mut(self).strat_decode(source, video_frame_handler)
	}
}

#[cfg(feature = "slint")]
impl Gui for Container {
	fn generate_video_frame_handler(&self) -> Box<dyn VideoFrameHandler> {
		SlintGui::inj_ref(self).generate_video_frame_handler()
	}

	fn run(&self) -> Result<()> { SlintGui::inj_ref(self).run() }
}
