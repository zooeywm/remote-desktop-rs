use super::pixel_format::PixelFormat;

/// Video Frame, because all of its fields are Send, it is Send
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct VideoInfo {
	/// Video width
	pub width: u32,

	/// Video height
	pub height: u32,

	/// Video pixel format
	pub format: PixelFormat,
}

impl VideoInfo {
	#[inline]
	pub fn new(width: u32, height: u32, format: PixelFormat) -> Self {
		Self { width, height, format }
	}

	/// Get the frame size represent for the video info.
	pub fn frame_size(&self) -> usize { self.format.frame_size(self.width, self.height) }

	#[inline]
	pub fn width(&self) -> u32 { self.width }

	#[inline]
	pub fn height(&self) -> u32 { self.height }

	#[inline]
	pub fn pixel_format(&self) -> PixelFormat { self.format }

	pub fn stride(&self) -> usize { self.format.stride(self.width) }
}
