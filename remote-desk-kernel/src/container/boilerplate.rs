use remote_desk_core::{error::Result, model::{StreamSource, VideoFrame}, service::Transcoder};
use remote_desk_codec::FFmpegCodec;

use crate::container::Container;

impl Transcoder for Container {
	fn strat_decode(
		&mut self,
		source: StreamSource,
		on_video_frame: impl Fn(&dyn VideoFrame) -> Result<()> + Send + 'static,
	) -> Result<()> {
		FFmpegCodec::inj_ref_mut(self).strat_decode(source, on_video_frame)
	}
}
