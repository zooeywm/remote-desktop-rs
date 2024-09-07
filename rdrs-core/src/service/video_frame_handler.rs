use crate::{error::Result, model::VideoFrame};

/// Video frame handler
pub trait VideoFrameHandler: Send + 'static {
	/// Handle video frame
	fn handle_video_frame(&self, video_frame: &dyn VideoFrame) -> Result<()>;
}
