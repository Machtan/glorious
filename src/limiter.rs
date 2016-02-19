//! Contains the frame limiter.

use std::time::{Duration, Instant};
use std::thread;

pub struct FrameLimiter {
    last_tick: Instant,
    max_fps: u32,
    frame_length: Duration,
    
}

impl FrameLimiter {
    /// Creates a new frame limiter limited to the given amount of frames per
    /// second.
    pub fn new(max_fps: u32) -> FrameLimiter {
        let frame_length = Duration::new(0, 1_000_000_000 / max_fps);
        FrameLimiter {
            last_tick: Instant::now(),
            max_fps: max_fps,
            frame_length: frame_length,
        }
    }
    
    /// Resets the limiter.
    pub fn reset(&mut self) {
        self.last_tick = Instant::now();
    }
    
    /// Sleeps until the remainder of the frame length of the set FPS.
    pub fn limit(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_from_earlier(self.last_tick);
        if elapsed < self.frame_length {
            let remainder = self.frame_length - elapsed;
            thread::sleep(remainder);
        } else {
            let nanos = elapsed.as_secs() * 1_000_000_000 
                + elapsed.subsec_nanos() as u64;
            let fps = 1_000_000_000 as f64 / nanos as f64;
            //println!("Too slow... Fps: {:05.2}", fps);
        }
        self.last_tick = Instant::now();
        
    }
}