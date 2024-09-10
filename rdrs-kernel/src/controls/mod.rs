use rdrs_core::{error::Result, model::StreamType, service::{CodecManager, Gui}};

use crate::container::Container;

impl Container {
	/// Prepare gui and generate VideoFrameHandler, then start decoding, the
	/// VideoFrameHandler will handle every decoded frame.
	pub fn start_decoding_streaming(&self, stream_source: StreamType) -> Result<()> {
		self.start_decode(stream_source)
	}

	pub fn stop_codec_by_id(&self, id: u8) -> Result<()> { self.close_by_id(id) }

	pub fn stop_all_codecs(&self) -> Result<()> { self.close_all() }

	pub fn start_gui(&self) -> Result<()> { self.run() }
}
