use std::{path::PathBuf, sync::atomic::AtomicU32};

use i_slint_backend_winit::{WinitWindowAccessor, WinitWindowEventResult};
use rdrs_kernel::{model::{StreamSource, VideoFrame}, Result};
use rdrs_slint::init;
use tokio::time::Instant;

slint::include_modules!();

#[tokio::main(worker_threads = 32)]
async fn main() -> Result<()> {
	let (mut container, app) = init()?;

	let window = app.window();
	// Just for a show case, no use here.
	window.on_winit_window_event(move |_window, event| {
		tracing::debug!("{event:?}");
		WinitWindowEventResult::Propagate
	});

	let video_path =
		container.extends.take().unwrap().into_table()?.get("video_path").unwrap().to_string();

	let result = container.start_decode(StreamSource::File { path: PathBuf::from(video_path) }, {
		let app_weak = app.as_weak();

		let count = std::sync::Arc::new(AtomicU32::new(0));
		let inst = Instant::now();
		move |frame| {
			let pixel_buffer = video_frame_to_pixel_buffer(frame);
			let count = count.clone();
			if let Err(err) = app_weak.upgrade_in_event_loop(move |app| {
				count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
				app.set_video_frame(slint::Image::from_rgb8(pixel_buffer));
				let count = count.load(std::sync::atomic::Ordering::Relaxed);
				tracing::debug!("{}:{}", count, inst.elapsed().as_millis());
			}) {
				tracing::error!("{err}")
			}
			Ok(())
		}
	});

	if let Err(err) = result {
		tracing::error!("{err}");
	};

	app.run()?;
	Ok(())
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
