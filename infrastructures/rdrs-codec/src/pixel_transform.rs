use ffmpeg_next::format::Pixel;
use rdrs_domain_player::vo::PixelFormat;

pub(super) fn ffmpeg_pixel_to_pixel(pixel: Pixel) -> PixelFormat {
	use Pixel::*;
	match pixel {
		// RGB24=>
		YUV420P => PixelFormat::Yuv420P,
		YUYV422 => PixelFormat::Yuyv422,
		YUV422P => PixelFormat::Yuv422P,
		YUV410P => PixelFormat::Yuv410P,
		YUV411P => PixelFormat::Yuv411P,
		RGB24 => PixelFormat::Rgb24,
		_ => PixelFormat::Unknown,
	}
}

pub(super) fn pixel_to_ffmpeg_pixel(pixel: PixelFormat) -> Pixel {
	use PixelFormat::*;
	match pixel {
		Yuv420P => Pixel::YUV420P,
		Yuyv422 => Pixel::YUYV422,
		Yuv422P => Pixel::YUV422P,
		Yuv410P => Pixel::YUV410P,
		Yuv411P => Pixel::YUV411P,
		Rgb24 => Pixel::RGB24,
		_ => Pixel::None,
	}
}
