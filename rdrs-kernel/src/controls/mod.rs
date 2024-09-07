use rdrs_core::{error::Result, model::StreamSource, service::{Codec, Gui}};

use crate::container::Container;

impl Container {
	/// Prepare gui and generate VideoFrameHandler, then start decoding, the
	/// VideoFrameHandler will handle every decoded frame.
	pub fn start_decode(&mut self, stream_source: StreamSource) -> Result<()> {
		let video_frame_handler = self.generate_video_frame_handler();
		self.strat_decode(stream_source, video_frame_handler)
	}

	pub fn start_gui(&self) -> Result<()> { self.run() }
}
