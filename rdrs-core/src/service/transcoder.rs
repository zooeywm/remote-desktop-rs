use crate::{error::Result, model::{StreamSource, VideoFrame}};

/// Use for decode stream
/// TODO: Split render logic from it
pub trait Transcoder {
	/// Decode frame, render asynchronously, and display by callback.
	fn strat_decode(
		&mut self,
		source: StreamSource,
		on_video_frame: impl Fn(&dyn VideoFrame) -> Result<()> + Send + 'static,
	) -> Result<()>;
}
