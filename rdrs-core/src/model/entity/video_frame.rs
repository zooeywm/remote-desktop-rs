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
