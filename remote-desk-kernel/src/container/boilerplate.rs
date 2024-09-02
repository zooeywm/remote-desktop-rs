use anyhow::Result;
use remote_desk_core::{model::StreamSource, service::StreamDecoder};
use remote_desk_decoder::FFmpegWithRodioStreamDecoder;

use crate::container::Container;

impl StreamDecoder for Container {
	fn init(&self) -> Result<()> { FFmpegWithRodioStreamDecoder::inj_ref(self).init() }

	fn handle_stream(&self, source: StreamSource) -> Result<()> {
		FFmpegWithRodioStreamDecoder::inj_ref(self).handle_stream(source)
	}
}
