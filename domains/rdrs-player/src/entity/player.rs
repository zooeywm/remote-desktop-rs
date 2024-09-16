use crate::vo::{StreamSource, VideoInfo};

/// Decode and render the stream.
pub struct Player {
	pub(crate) stream_source:     StreamSource,
	pub(crate) render_video_info: VideoInfo,
}

impl Player {
	#[inline]
	pub fn new(stream_source: StreamSource, render_video_info: VideoInfo) -> Self {
		Self { stream_source, render_video_info }
	}
}
