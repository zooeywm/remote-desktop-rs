mod container;
mod controls;
mod implements;
mod infrastructure;

pub use container::Container;
pub use remote_desk_config::AppConfig;
pub use remote_desk_core::model::StreamSource;

type OnVideoFrame = dyn Fn(&FFmpegVideoFrame) -> anyhow::Result<()> + Send;
pub type FFmpegVideoFrame = ffmpeg_next::frame::Video;
type CodecContext = ffmpeg_next::codec::Context;
type VideoDecoder = ffmpeg_next::decoder::Video;
type ScalingContext = ffmpeg_next::software::scaling::Context;
type FFmpegError = ffmpeg_next::Error;
