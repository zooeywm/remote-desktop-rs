use anyhow::Result;
use remote_desk_core::{model::StreamSource, service::StreamDecoder};

use crate::container::Container;

impl Container {
	pub fn start_decode(&self, stream_source: StreamSource) -> Result<()> {
		self.init().unwrap();
		self.handle_stream(stream_source)
	}
}
