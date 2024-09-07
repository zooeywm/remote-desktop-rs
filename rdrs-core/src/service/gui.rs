use super::VideoFrameHandler;
use crate::error::Result;

/// Gui service
pub trait Gui {
	/// Generate a VideoFrameHandler, will handle the video frame to display
	fn generate_video_frame_handler(&self) -> Box<dyn VideoFrameHandler>;

	/// Block to run the app (usually on main thread).
	fn run(&self) -> Result<()>;
}
