use crate::{error::Result, model::{StreamType, VideoStreamInfo}};

/// Use for decode stream
/// TODO: Split render logic from it
pub trait CodecManager {
	/// Start a new thread, decode frame, render, and display by callback.
	fn start_decode(&self, source: StreamType) -> Result<()>;

	/// Close codec by id
	fn close_by_id(&self, id: u8) -> Result<()>;

	/// Close all codecs
	fn close_all(&self) -> Result<()>;

	/// Update Video steam by id, if nothing to change, return Ok(false)
	fn update_video_stream_by_id(&self, id: u8, new_info: VideoStreamInfo) -> Result<bool>;
}
