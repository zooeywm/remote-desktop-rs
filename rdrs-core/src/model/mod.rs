mod pixel;

use std::path::PathBuf;

pub use pixel::Pixel;

#[non_exhaustive]
#[derive(Debug)]
pub enum StreamType {
	File { path: PathBuf },
}

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

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
/// P is for Pixel format
pub struct VideoStreamInfo {
	/// Pixel format
	pub format: Pixel,
	pub width:  u32,
	pub height: u32,
}
