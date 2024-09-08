
pub(crate) struct StreamClock {
    time_base_seconds: f64,
    start_time: std::time::Instant,
}

impl StreamClock {
    pub fn new(stream: &ffmpeg_next::format::stream::Stream) -> Self {
        let time_base_seconds = stream.time_base();
        let time_base_seconds =
            time_base_seconds.numerator() as f64 / time_base_seconds.denominator() as f64;

        let start_time = std::time::Instant::now();

        Self { time_base_seconds, start_time }
    }

    pub fn convert_pts_to_instant(&self, pts: Option<i64>) -> Option<std::time::Duration> {
        pts.and_then(|pts| {
            let pts_since_start =
                std::time::Duration::from_secs_f64(pts as f64 * self.time_base_seconds);
            self.start_time.checked_add(pts_since_start)
        })
        .map(|absolute_pts| absolute_pts.duration_since(std::time::Instant::now()))
    }
}
