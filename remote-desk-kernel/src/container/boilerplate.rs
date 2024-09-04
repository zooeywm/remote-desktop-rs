use remote_desk_core::{error::Result, model::{StreamSource, VideoFrame}, service::StreamDecoder};
use remote_desk_decoder::FFmpegWithRodioStreamDecoder;

use crate::container::Container;

impl StreamDecoder for Container {
	fn decode_stream(
		&self,
		source: StreamSource,
		on_video_frame: impl Fn(&dyn VideoFrame) -> Result<()> + Send + 'static,
	) -> Result<()> {
		FFmpegWithRodioStreamDecoder::inj_ref(self).decode_stream(source, on_video_frame)
	}
}
