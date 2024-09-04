use remote_desk_core::{error::Result, model::{StreamSource, VideoFrame}, service::StreamDecoder};

use crate::container::Container;

impl Container {
	pub fn start_decode(
		&self,
		stream_source: StreamSource,
		on_video_frame: impl Fn(&dyn VideoFrame) -> Result<()> + Send + 'static,
	) -> Result<()> {
		self.decode_stream(stream_source, on_video_frame)
	}
}
