use std::time::{Duration, Instant};
use std::thread;

/// A frame limiter with a maximum frame rate.
pub struct FrameLimiter {
    last_tick: Instant,
    //max_fps: u32,
    frame_length: Duration,
}

impl FrameLimiter {
    /// Creates a new frame limiter.
    ///
    /// The game will run with a frame rate of at most `max_fps` frames
    /// per second.
    pub fn new(max_fps: u32) -> FrameLimiter {
        let frame_length = Duration::new(0, 1_000_000_000 / max_fps);
        FrameLimiter {
            last_tick: Instant::now(),
            //max_fps: max_fps,
            frame_length: frame_length,
        }
    }

    /// Resets the limiter.
    pub fn reset(&mut self) {
        self.last_tick = Instant::now();
    }

    /// Sleeps until the end of the current frame.
    pub fn limit(&mut self) {
        let elapsed = self.last_tick.elapsed();
        if elapsed < self.frame_length {
            let remainder = self.frame_length - elapsed;
            thread::sleep(remainder);
        } else {
            //let nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
            //let fps = 1_000_000_000 as f64 / nanos as f64;
            //println!("Too slow... FPS: {:05.2}", fps);
        }
        self.last_tick = Instant::now();
    }
}
