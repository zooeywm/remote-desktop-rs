use std::sync::atomic::{AtomicU8, Ordering};

use dashmap::DashMap;
use rdrs_core::{error::Result, model::StreamType, service::{Codec, VideoFrameHandlerGenerator}};
use tracing::error;

use crate::codec::FFmpegCodec;

#[derive(dep_inj::DepInj, Default)]
#[target(FFmpegCodecManager)]
pub struct FFmpegCodecManagerState {
	current_source_id: AtomicU8,
	codecs:            DashMap<u8, FFmpegCodec>,
}

impl FFmpegCodecManagerState {
	pub fn new() -> Result<Self> {
		ffmpeg_next::init()?;
		Ok(Self::default())
	}
}

impl<Deps> Codec for FFmpegCodecManager<Deps>
where
	Deps: AsRef<FFmpegCodecManagerState> + VideoFrameHandlerGenerator,
{
	fn start_decode(&self, source: StreamType) -> Result<()> {
		let id = self.current_source_id.fetch_add(1, Ordering::Relaxed);
		let video_frame_handler = self.prj_ref().generate_video_frame_handler();
		let codec = FFmpegCodec::start(source, video_frame_handler)?;
		self.codecs.insert(id, codec);
		Ok(())
	}

	fn close_by_id(&self, id: u8) -> Result<()> {
		match self.codecs.remove(&id) {
			Some(_) => {
				tracing::info!("Stopped codec: {id}.");
			}
			None => error!("No such codec with id: {id}."),
		}
		Ok(())
	}

	fn close_all(&self) -> Result<()> {
		self.codecs.clear();
		tracing::info!("Stopped all codecs.");
		Ok(())
	}
}
