mod pixel;

pub use pixel::Pixel;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
/// P is for Pixel format
pub struct VideoStreamInfo {
	/// Pixel format
	pub format: Pixel,
	pub width:  u32,
	pub height: u32,
}
