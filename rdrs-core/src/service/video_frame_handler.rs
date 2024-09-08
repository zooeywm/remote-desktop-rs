use crate::{error::Result, model::VideoFrame};

/// Video frame handler
pub trait VideoFrameHandlerGenerator {
	/// Generate a VideoFrameHandler, will handle the video frame to display
	fn generate_video_frame_handler(&self) -> Box<dyn VideoFrameHandler>;
}

pub trait VideoFrameHandler: Send + 'static {
	/// Handle video frame
	fn handle_video_frame(&self, video_frame: &dyn VideoFrame) -> Result<()>;
}
