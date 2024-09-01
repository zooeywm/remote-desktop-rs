use anyhow::Result;
use remote_desk_core::{model::StreamSource, service::StreamDecoder};

use crate::{container::Container, implements::FFmpegWithRodioStreamDecoder};

impl StreamDecoder for Container {
	fn init(&self) -> Result<()> { FFmpegWithRodioStreamDecoder::inj_ref(self).init() }

	fn handle_stream(&self, source: StreamSource) -> Result<()> {
		FFmpegWithRodioStreamDecoder::inj_ref(self).handle_stream(source)
	}
}
