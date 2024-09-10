use crate::{error::Result, model::{StreamType, VideoStreamInfo}, service::VideoFrameHandler};

pub trait Codec {
	type C;

	/// Init the environment, this should be called only once.
	fn init() -> Result<()>;

	/// Start decode with stream source, and call VideoFrameHandler on every
	/// video_frame.
	fn start(source: StreamType, video_frame_handler: Box<dyn VideoFrameHandler>) -> Result<Self::C>;

	/// CHange the video info.
	fn change_video_info(&self, new_info: VideoStreamInfo) -> bool;
}
