/// Video Frame, because all of its fields are Send, it is Send
#[derive(Debug, Default, Clone)]
pub struct VideoFrame {
	pub data:   Box<[u8]>,
	pub width:  u32,
	pub height: u32,
}
