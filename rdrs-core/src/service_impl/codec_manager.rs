use std::sync::atomic::{AtomicU8, Ordering};

use dashmap::DashMap;
use dep_inj::DepInj;
use tracing::{error, info};

use crate::{error::Result, infra_trait::Codec, model::VideoStreamInfo, service::{CodecManager, VideoFrameHandlerGenerator}};

#[derive(DepInj, Default)]
#[target(CodecManagerImpl)]
pub struct CodecManagerState<C> {
	current_source_id: AtomicU8,
	codecs:            DashMap<u8, C>,
}

impl<C: Codec<C = C>> CodecManagerState<C> {
	pub fn new() -> Result<Self> {
		C::init()?;
		Ok(Self { current_source_id: AtomicU8::new(0), codecs: DashMap::new() })
	}
}

impl<C, Deps> CodecManager for CodecManagerImpl<C, Deps>
where
	C: Codec<C = C>,
	Deps: AsRef<CodecManagerState<C>> + VideoFrameHandlerGenerator,
{
	fn start_decode(&self, source: crate::model::StreamType) -> Result<()> {
		let id = self.current_source_id.fetch_add(1, Ordering::Relaxed);
		let video_frame_handler = self.prj_ref().generate_video_frame_handler();
		let codec = C::start(source, video_frame_handler)?;
		self.codecs.insert(id, codec);
		Ok(())
	}

	fn close_by_id(&self, id: u8) -> Result<()> {
		match self.codecs.remove(&id) {
			Some(_) => {
				info!("Stopped codec: {id}.");
			}
			None => error!("No such codec with id: {id}."),
		}
		Ok(())
	}

	fn close_all(&self) -> Result<()> {
		self.codecs.clear();
		info!("Stopped all codecs.");
		Ok(())
	}

	fn update_video_stream_by_id(&self, id: u8, new_info: VideoStreamInfo) -> Result<bool> {
		match self.codecs.get(&id) {
			Some(codec) => {
				return Ok(codec.change_video_info(new_info));
			}
			None => error!("No such codec with id: {id}."),
		}
		Ok(false)
	}
}
