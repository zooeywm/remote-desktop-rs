use super::VideoFrameHandler;
use crate::{error::Result, model::StreamSource};

/// Use for decode stream
/// TODO: Split render logic from it
pub trait Codec {
	/// Start a new thread, decode frame, render, and display by callback.
	fn strat_decode(
		&mut self,
		source: StreamSource,
		video_frame_handler: Box<dyn VideoFrameHandler>,
		// on_video_frame: impl Fn(&(dyn VideoFrame + '_)) -> Result<()> + Send + 'static,
	) -> Result<()>;
}
