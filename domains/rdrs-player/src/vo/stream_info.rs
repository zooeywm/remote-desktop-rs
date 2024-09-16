use std::path::PathBuf;

/// Stream source.
#[derive(Debug, Clone)]
pub enum StreamSource {
	/// Stream source is a file.
	File {
		/// File path
		path: PathBuf,
	},
}

pub struct Rational {
	numerator:   i32,
	denominator: i32,
}

impl Rational {
	pub fn new(numerator: i32, denominator: i32) -> Self { Self { numerator, denominator } }
}

pub struct StreamClock {
	time_base_seconds: f64,
	start_time:        std::time::Instant,
}

impl StreamClock {
	pub fn new(Rational { numerator, denominator }: Rational) -> Self {
		let time_base_seconds = numerator as f64 / denominator as f64;

		let start_time = std::time::Instant::now();

		Self { time_base_seconds, start_time }
	}

	pub fn convert_pts_to_instant(&self, pts: Option<i64>) -> Option<std::time::Duration> {
		pts
			.and_then(|pts| {
				let pts_since_start =
					std::time::Duration::from_secs_f64(pts as f64 * self.time_base_seconds);
				self.start_time.checked_add(pts_since_start)
			})
			.map(|absolute_pts| absolute_pts.duration_since(std::time::Instant::now()))
	}
}
