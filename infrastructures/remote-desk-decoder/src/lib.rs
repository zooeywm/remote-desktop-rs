mod stream_decoder;

type OnVideoFrame = dyn Fn(&dyn VideoFrame) -> anyhow::Result<()> + Send;
type FFmpegVideoFrame = ffmpeg_next::frame::Video;
type CodecContext = ffmpeg_next::codec::Context;
type VideoDecoder = ffmpeg_next::decoder::Video;
type ScalingContext = ffmpeg_next::software::scaling::Context;
type FFmpegError = ffmpeg_next::Error;

pub use stream_decoder::{FFmpegWithRodioStreamDecoder, FFmpegWithRodioStreamDecoderState};

/// Video Frame, because all of its fields are Send, it is Send
pub trait VideoFrame: Send {
	/// Frame width
	fn width(&self) -> u32;

	/// Frame height
	fn height(&self) -> u32;

	/// Frame data of index
	fn data(&self, index: usize) -> &[u8];

	/// Frame stride of index
	fn stride(&self, index: usize) -> usize;
}

impl VideoFrame for FFmpegVideoFrame {
	fn width(&self) -> u32 { self.width() }

	fn height(&self) -> u32 { self.height() }

	fn data(&self, index: usize) -> &[u8] { self.data(index) }

	fn stride(&self, index: usize) -> usize { self.stride(index) }
}
