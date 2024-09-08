use std::ops::Deref;

use rdrs_core::model::VideoFrame;

pub(crate) struct FFmpegVideoFrame<'a>(pub &'a ffmpeg_next::frame::Video);

impl Deref for FFmpegVideoFrame<'_> {
	type Target = ffmpeg_next::frame::Video;

	fn deref(&self) -> &Self::Target { self.0 }
}

impl VideoFrame for FFmpegVideoFrame<'_> {
	fn width(&self) -> u32 { self.0.width() }

	fn height(&self) -> u32 { self.0.height() }

	fn data(&self, index: usize) -> &[u8] { self.0.data(index) }

	fn stride(&self, index: usize) -> usize { self.0.stride(index) }
}
