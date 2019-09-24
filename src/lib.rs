//! Rhythm game synchronization tool

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct AudioTracker {
    prior_ftime: Instant,
    prior_phead: u32,
    song_time: Duration,
    visual_lag: i32,
    audio_lag: i32,
}

impl AudioTracker {
    pub fn new(vlag: i32, alag: i32) -> Self {
        Self {
            prior_ftime: Instant::now(),
            prior_phead: 0,
            song_time: Duration::new(0, 0),
            audio_lag: alag,
            visual_lag: vlag,
        }
    }

    /// Only use when receiving a fresh playhead report
    pub fn ease(&mut self, playhead: u32) {
        let now = Instant::now();

        self.song_time = now.duration_since(self.prior_ftime);
        self.prior_ftime = now;

        if self.prior_phead != playhead {
            self.song_time = self.song_time + Duration::new(0, playhead);
            self.prior_phead = playhead;
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
