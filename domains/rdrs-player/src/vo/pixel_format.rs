#[derive(Eq, PartialEq, Debug, Clone, Copy, Default)]
pub enum PixelFormat {
	Yuv420P,
	Yuyv422,
	Yuv422P,
	Yuv410P,
	Yuv411P,
	Rgb24,
	#[default]
	Unknown,
}

impl PixelFormat {
	/// Calculate frame size with width and height represent for the pixel format
	pub fn frame_size(&self, width: u32, height: u32) -> usize {
		use PixelFormat::*;

		let size = match self {
			Yuv422P => yuv_size(width, height, 2),
			Yuv420P | Yuv411P => yuv_size(width, height, 4),
			Yuv410P => yuv_size(width, height, 16),
			Yuyv422 => width * height * 2,
			_ => {
				unimplemented!()
			}
		};
		size as usize
	}

	/// Caculate frame stride
	pub fn stride(&self, width: u32) -> usize {
		use PixelFormat::*;

		let size = match self {
			Rgb24 => width * 3,
			_ => {
				unimplemented!()
			}
		};
		size as usize
	}
}

#[inline]
fn yuv_size(width: u32, height: u32, colorimetric_ratio: u32) -> u32 {
	let y_size = width * height;
	let u_size = y_size.div_ceil(colorimetric_ratio);
	let v_size = u_size;
	y_size + u_size + v_size
}
