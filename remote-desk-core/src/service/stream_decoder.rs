use anyhow::Result;

use crate::model::StreamSource;

/// Use for decode stream
/// TODO: Split render logic from it
pub trait StreamDecoder {
	/// Init the StreamDecoder
	fn init(&self) -> Result<()>;

	/// Decode frame synchronously, render asynchronously, display synchronously.
	/// decode --> render
	/// render -> display
	/// Thus need a async channel to synchronize from render to display
	fn handle_stream(&self, source: StreamSource) -> Result<()>;
}
