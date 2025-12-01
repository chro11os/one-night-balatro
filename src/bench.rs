use std::time::{Duration, Instant};
use std::collections::VecDeque;

pub struct GameBench {
    pub frame_times: VecDeque<Duration>,
    pub update_times: VecDeque<Duration>,
    pub draw_times: VecDeque<Duration>,
    pub last_report: Instant,
    pub frame_count: u32,
    pub updates_per_second: f64,
}

impl GameBench {
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(600), // Store last ~10 seconds at 60fps
            update_times: VecDeque::with_capacity(600),
            draw_times: VecDeque::with_capacity(600),
            last_report: Instant::now(),
            frame_count: 0,
            updates_per_second: 0.0,
        }
    }

    pub fn start_frame(&mut self) -> Instant {
        Instant::now()
    }

    pub fn end_frame(&mut self, start: Instant) {
        let duration = start.elapsed();
        self.frame_times.push_back(duration);
        if self.frame_times.len() > 600 {
            self.frame_times.pop_front();
        }
        self.frame_count += 1;
    }

    pub fn record_update(&mut self, duration: Duration) {
        self.update_times.push_back(duration);
        if self.update_times.len() > 600 {
            self.update_times.pop_front();
        }
    }

    pub fn record_draw(&mut self, duration: Duration) {
        self.draw_times.push_back(duration);
        if self.draw_times.len() > 600 {
            self.draw_times.pop_front();
        }
    }

    pub fn report(&mut self) -> Option<String> {
        if self.last_report.elapsed() >= Duration::from_secs(1) {
            let elapsed = self.last_report.elapsed().as_secs_f64();
            self.updates_per_second = self.frame_count as f64 / elapsed;

            let avg_frame = self.avg_duration(&self.frame_times);
            let avg_update = self.avg_duration(&self.update_times);
            let avg_draw = self.avg_duration(&self.draw_times);

            let max_fps = 1.0 / avg_frame.as_secs_f64();

            let report = format!(
                "--- BENCHMARK REPORT ---\n\
                 FPS (Actual): {:.2}\n\
                 Avg Frame Time: {:.2?}\n\
                 Avg Update Time: {:.2?}\n\
                 Avg Draw Time: {:.2?}\n\
                 Est. Max FPS: {:.2}\n\
                 Bottleneck: {}\n\
                 ------------------------",
                self.updates_per_second,
                avg_frame,
                avg_update,
                avg_draw,
                max_fps,
                if avg_update > avg_draw { "Update (Logic)" } else { "Draw (Render)" }
            );

            self.frame_count = 0;
            self.last_report = Instant::now();
            return Some(report);
        }
        None
    }

    fn avg_duration(&self, times: &VecDeque<Duration>) -> Duration {
        if times.is_empty() {
            return Duration::ZERO;
        }
        let sum: Duration = times.iter().sum();
        sum / times.len() as u32
    }
}   