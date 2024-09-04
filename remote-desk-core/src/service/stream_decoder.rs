use crate::{error::Result, model::{StreamSource, VideoFrame}};

/// Use for decode stream
/// TODO: Split render logic from it
pub trait StreamDecoder {
	/// Decode frame synchronously, render asynchronously, display synchronously.
	/// decode --> render
	/// render -> display
	/// Thus need a async channel to synchronize from render to display
	fn decode_stream(
		&self,
		source: StreamSource,
		on_video_frame: impl Fn(&dyn VideoFrame) -> Result<()> + Send + 'static,
	) -> Result<()>;
}
