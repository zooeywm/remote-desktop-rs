use std::path::PathBuf;

use remote_desk_kernel::{model::StreamSource, AppConfig, Container, VideoFrame};

slint::include_modules!();

#[tokio::main]
async fn main() {
	let args: Vec<String> = std::env::args().collect();
	let video_path = args.get(1).expect("Please provide a video file path");
	let app = App::new().unwrap();
	let container = Container::new(&AppConfig::new(1), {
		let app_weak = app.as_weak();

		move |frame| {
			let pixel_buffer = video_frame_to_pixel_buffer(frame);
			app_weak
				.upgrade_in_event_loop(|app| app.set_video_frame(slint::Image::from_rgb8(pixel_buffer)))
				.unwrap();
			Ok(())
		}
	});
	container.start_decode(StreamSource::File { path: PathBuf::from(video_path) }).unwrap();

	app.run().unwrap();
}

pub fn video_frame_to_pixel_buffer(
	frame: &(impl VideoFrame + ?Sized),
) -> slint::SharedPixelBuffer<slint::Rgb8Pixel> {
	let mut pixel_buffer =
		slint::SharedPixelBuffer::<slint::Rgb8Pixel>::new(frame.width(), frame.height());

	let ffmpeg_line_iter = frame.data(0).chunks_exact(frame.stride(0));
	let slint_pixel_line_iter = pixel_buffer
		.make_mut_bytes()
		.chunks_mut(frame.width() as usize * core::mem::size_of::<slint::Rgb8Pixel>());

	for (source_line, dest_line) in ffmpeg_line_iter.zip(slint_pixel_line_iter) {
		dest_line.copy_from_slice(&source_line[..dest_line.len()])
	}

	pixel_buffer
}
