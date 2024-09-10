use rdrs_codec::FFmpegCodec;
use rdrs_core::{error::Result, model::{StreamType, VideoStreamInfo}, service::{CodecManager, Gui, VideoFrameHandler, VideoFrameHandlerGenerator}, service_impl::CodecManagerImpl};
use rdrs_gui::SlintGui;

use crate::container::Container;

impl CodecManager for Container {
	fn start_decode(&self, source: StreamType) -> Result<()> {
		CodecManagerImpl::<FFmpegCodec, _>::inj_ref(self).start_decode(source)
	}

	fn close_by_id(&self, id: u8) -> Result<()> {
		CodecManagerImpl::<FFmpegCodec, _>::inj_ref(self).close_by_id(id)
	}

	fn close_all(&self) -> Result<()> {
		CodecManagerImpl::<FFmpegCodec, _>::inj_ref(self).close_all()
	}

	fn update_video_stream_by_id(&self, id: u8, new_info: VideoStreamInfo) -> Result<bool> {
		CodecManagerImpl::<FFmpegCodec, _>::inj_ref(self).update_video_stream_by_id(id, new_info)
	}
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
