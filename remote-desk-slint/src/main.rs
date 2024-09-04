use std::{io::Write, path::PathBuf, sync::atomic::AtomicU32, time::Instant};

use i_slint_backend_winit::{WinitWindowAccessor, WinitWindowEventResult};
use remote_desk_kernel::{model::{StreamSource, VideoFrame}, AppConfig, Container};

slint::include_modules!();

#[tokio::main]
async fn main() {
	env_logger::Builder::new()
		.format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
		.filter(Some("remote_desk_slint"), log::LevelFilter::Info)
		.init();

	std::panic::set_hook(Box::new(|panic_info| {
		log::error!("panic occurred: {panic_info}");
	}));

	let args: Vec<String> = std::env::args().collect();
	slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap())).unwrap();
	let video_path = args.get(1).expect("Please provide a video file path");
	let app = App::new().unwrap();
	let window = app.window();

	window.on_winit_window_event(move |_window, event| {
		log::debug!("{event:?}");
		WinitWindowEventResult::Propagate
	});

	let container = Container::new(&AppConfig::new(1));

	let result = container.start_decode(StreamSource::File { path: PathBuf::from(video_path) }, {
		let app_weak = app.as_weak();

		let count = std::sync::Arc::new(AtomicU32::new(0));
		let inst = Instant::now();
		move |frame| {
			let pixel_buffer = video_frame_to_pixel_buffer(frame);
			let count = count.clone();
			app_weak
				.upgrade_in_event_loop(move |app| {
					count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

					app.set_video_frame(slint::Image::from_rgb8(pixel_buffer));
					let count = count.load(std::sync::atomic::Ordering::Relaxed);
					log::info!("{}:{}", count, inst.elapsed().as_millis());
				})
				.unwrap();
			Ok(())
		}
	});

	if let Err(err) = result {
		log::error!("{err}");
	};

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
