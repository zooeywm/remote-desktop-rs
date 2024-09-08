use super::VideoFrameHandler;
use crate::{error::Result, model::StreamType};

/// Use for decode stream
/// TODO: Split render logic from it
pub trait Codec {
	/// Start a new thread, decode frame, render, and display by callback.
	fn start_decode(
		&self,
		source: StreamType,
		video_frame_handler: Box<dyn VideoFrameHandler>,
	) -> Result<()>;

	/// Close codec by id
	fn close_by_id(&self, id: u8) -> Result<()>;

	/// Close all codecs
	fn close_all(&self) -> Result<()>;
}
