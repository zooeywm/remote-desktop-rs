mod stream_decoder;

type CodecContext = ffmpeg_next::codec::Context;
type VideoDecoder = ffmpeg_next::decoder::Video;
type ScalingContext = ffmpeg_next::software::scaling::Context;
type FFmpegError = ffmpeg_next::Error;

use std::ops::{Deref, DerefMut};

use remote_desk_core::model::VideoFrame;
pub use stream_decoder::FFmpegWithRodioStreamDecoder;

struct FFmpegVideoFrame(ffmpeg_next::frame::Video);

impl FFmpegVideoFrame {
	fn empty() -> Self { Self(ffmpeg_next::frame::Video::empty()) }
}

impl Deref for FFmpegVideoFrame {
	type Target = ffmpeg_next::frame::Video;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for FFmpegVideoFrame {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl VideoFrame for FFmpegVideoFrame {
	fn width(&self) -> u32 { self.0.width() }

	fn height(&self) -> u32 { self.0.height() }

	fn data(&self, index: usize) -> &[u8] { self.0.data(index) }

	fn stride(&self, index: usize) -> usize { self.0.stride(index) }
}
