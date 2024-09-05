use remote_desk_core::{error::Result, model::{StreamSource, VideoFrame}, service::Transcoder};

use crate::container::Container;

impl Container {
    /// Start decoding, call on_video_frame every decoded frame.
	pub fn start_decode(
		&mut self,
		stream_source: StreamSource,
		on_video_frame: impl Fn(&dyn VideoFrame) -> Result<()> + Send + 'static,
	) -> Result<()> {
		self.strat_decode(stream_source, on_video_frame)
	}
}
