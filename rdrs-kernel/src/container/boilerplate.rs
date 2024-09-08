use rdrs_codec::FFmpegCodecManager;
use rdrs_core::{error::Result, model::StreamType, service::{Codec, Gui, VideoFrameHandler, VideoFrameHandlerGenerator}};
use rdrs_gui::SlintGui;

use crate::container::Container;

impl Codec for Container {
	fn start_decode(&self, source: StreamType) -> Result<()> {
		FFmpegCodecManager::inj_ref(self).start_decode(source)
	}

	fn close_by_id(&self, id: u8) -> Result<()> { FFmpegCodecManager::inj_ref(self).close_by_id(id) }

	fn close_all(&self) -> Result<()> { FFmpegCodecManager::inj_ref(self).close_all() }
}

#[cfg(feature = "slint")]
impl Gui for Container {
	fn run(&self) -> Result<()> { SlintGui::inj_ref(self).run() }
}

#[cfg(feature = "slint")]
impl VideoFrameHandlerGenerator for Container {
	fn generate_video_frame_handler(&self) -> Box<dyn VideoFrameHandler> {
		SlintGui::inj_ref(self).generate_video_frame_handler()
	}
}
