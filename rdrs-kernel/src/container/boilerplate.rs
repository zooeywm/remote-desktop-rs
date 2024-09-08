use rdrs_codec::FFmpegCodecManager;
use rdrs_core::{error::Result, model::StreamType, service::{Codec, Gui, VideoFrameHandler}};
use rdrs_gui::SlintGui;

use crate::container::Container;

impl Codec for Container {
	fn start_decode(
		&self,
		source: StreamType,
		video_frame_handler: Box<dyn VideoFrameHandler>,
	) -> Result<()> {
		FFmpegCodecManager::inj_ref(self).start_decode(source, video_frame_handler)
	}

	fn close_by_id(&self, id: u8) -> Result<()> { FFmpegCodecManager::inj_ref(self).close_by_id(id) }

	fn close_all(&self) -> Result<()> { FFmpegCodecManager::inj_ref(self).close_all() }
}

#[cfg(feature = "slint")]
impl Gui for Container {
	fn generate_video_frame_handler(&self) -> Box<dyn VideoFrameHandler> {
		SlintGui::inj_ref(self).generate_video_frame_handler()
	}

	fn run(&self) -> Result<()> { SlintGui::inj_ref(self).run() }
}
