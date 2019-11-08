//! Throttle events and record event stats with a simple library
//!
//! throttle_timer has no dependencies
//!
//! `ThrottleTimer` struct is created with a max frequency and label
//!
//! ```ThrottleTimer::new(Duration::from_secs(1_u64), &"Once every second");```
//!
//! Calling ```run()``` will check the last call time. If max frequency time has not passed the fn will return false.
//! If max_frequency duration has passed since the last call then the fn will return true
//!
//!
//! # Example
//! ```
//! use std::time::Duration;
//! use throttle_timer::ThrottleTimer;
//!
//! let mut break_timer = ThrottleTimer::new(Duration::from_secs(10_u64), &"Break");
//! let mut val = 0_u8;
//!
//! // timers always run when no previous runs
//! break_timer.run(&mut || val += 1);
//! for _ in 0..100 {
//!     // timer will not run as 10 secs has not passed
//!     // do run will return false
//!     break_timer.run(&mut || val += 1);
//! }
//!
//! break_timer.print_stats();
//! // Break called 0/sec, total calls 1, has been running for 10us
//!
//! assert_eq!(break_timer.total_calls(), &1);
//! assert_eq!(val, 1_u8);
//!
//!
//! ```

use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;

#[derive(Debug)]
pub struct ThrottleTimer {
    maybe_last_called_time: Option<Instant>,
    total_calls: usize,
    created_date: SystemTime,
    max_frequency: Duration,
    event_name: &'static str,
}

///
/// # Example
/// ```
/// use std::time::Duration;
/// use throttle_timer::ThrottleTimer;
///
/// let mut break_timer: ThrottleTimer = ThrottleTimer::new(Duration::from_secs(1_u64), &"Break");
/// let do_break_flag = break_timer.run(&mut || {});
///
/// // Timers always run when no previous runs
/// assert!(do_break_flag == true);
/// if do_break_flag {
///     println!("timer do run flag is set to true")
/// }
///
/// // Run flag false as no time has passed
/// assert!(break_timer.run(&mut || {}) == false);
/// ```
impl ThrottleTimer {
    pub fn new(max_frequency: std::time::Duration, event_name: &'static str) -> Self {
        Self {
            maybe_last_called_time: None,
            max_frequency,
            event_name,
            total_calls: 0,
            created_date: SystemTime::now(),
        }
    }
    pub const fn event_name(&self) -> &str {
        self.event_name
    }
    pub const fn total_calls(&self) -> &usize {
        &self.total_calls
    }
    pub const fn max_frequency(&self) -> &Duration {
        &self.max_frequency
    }
    pub const fn created_date(&self) -> SystemTime {
        self.created_date
    }
    pub fn wait_time(&self) -> Duration {
        match self.maybe_last_called_time {
            None => Duration::from_secs(0),
            Some(last_time) => {
                (self.max_frequency
                    - Instant::now()
                        .duration_since(last_time)
                        .min(self.max_frequency))
            }
        }
    }

    /// Prints total calls and calls/sec
    pub fn print_stats(&self) {
        match self.created_date.elapsed() {
            Ok(created_time_elapsed) => {
                println!(
                    "{} called {}/sec, total calls {}, has been running for {:?}",
                    self.event_name,
                    created_time_elapsed.as_secs() / self.total_calls as u64,
                    self.total_calls,
                    created_time_elapsed,
                );
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }

    /// Calling ```run()``` will check the last call time. If max frequency time has not passed the fn will return false.
    /// If max_frequency duration has passed since the last call then the fn will return true
    pub fn can_run(&mut self) -> bool {
        match self.maybe_last_called_time {
            None => true,
            Some(last_time) => Instant::now().duration_since(last_time) >= self.max_frequency,
        }
    }

    pub fn run_throttle_cb(&mut self, success: &mut FnMut(), throttled: &mut FnMut()) -> bool {
        let run_flag: bool = self.can_run();

        if run_flag {
            self.maybe_last_called_time = Some(Instant::now());
            self.total_calls += 1;
            success();
        } else {
            throttled()
        }
        run_flag
    }

    /// Calling ```run()``` will check the last call time. If max frequency time has not passed the fn will return false.
    /// If max_frequency duration has passed since the last call then the fn will return true
    pub fn run(&mut self, success: &mut FnMut()) -> bool {
        self.run_throttle_cb(success, &mut || {})
    }

    /// Calling ```run()``` will check the last call time. If max frequency time has not passed the fn will return false.
    /// If max_frequency duration has passed since the last call then the fn will return true
    pub fn run_wait(&mut self, success: &mut FnMut()) {
        thread::sleep(self.wait_time());
        self.run_throttle_cb(success, &mut || {});
    }

    // Same as run but will print a message if throttled
    pub fn run_with_msg(&mut self, success: &mut FnMut()) -> bool {
        let did_run = self.run(success);
        if !did_run {
            println!(
                "{} throttled, last time {:?}",
                self.event_name(),
                Instant::now().duration_since(self.maybe_last_called_time.unwrap())
            );
        }
        did_run
    }
}

#[cfg(test)]
mod test {
    use super::ThrottleTimer;
    use std::{thread, time::Duration};

    #[test]
    fn test_run() {
        let mut break_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_secs(45_000_u64), &"Break");
        let run_flag = break_timer.run(&mut || {});

        // timers always run when no previous runs
        assert!(run_flag);
        if run_flag {
            println!("timer do run flag is set to true")
        }
        break_timer.event_name();
        break_timer.total_calls();
        break_timer.max_frequency();
        break_timer.created_date();
    }

    #[test]
    fn test_run_with_msg() {
        let mut break_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_secs(45_000_u64), &"Break");
        let run_flag = break_timer.run_with_msg(&mut || {});

        // timers always run when no previous runs
        assert!(run_flag);
    }

    #[test]
    fn test_call_count() {
        let mut break_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_nanos(1_u64), &"Break");

        for _ in 0..100 {
            assert_eq!(break_timer.run(&mut || {}), true);
            thread::sleep(Duration::from_nanos(100_u64));
        }

        // timers always run when no previous runs
        assert_eq!(break_timer.total_calls, 100);
        break_timer.print_stats();
    }

    #[test]
    fn test_can_run() {
        let mut break_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_secs(1_u64), &"Break");

        assert!(break_timer.run(&mut || {}));
        for _ in 0..100 {
            assert!(!break_timer.can_run());
            assert!(!break_timer.run(&mut || {}));
        }

        // timers always run when no previous runs
        assert_eq!(break_timer.total_calls, 1);
        break_timer.print_stats();
    }

    #[test]
    fn test_print_debug() {
        println!(
            "{:?}",
            ThrottleTimer::new(Duration::from_nanos(1_u64), &"Break")
        );
    }

    #[test]
    fn test_in_loop() {
        let mut break_timer = ThrottleTimer::new(Duration::from_secs(10_u64), &"Break");

        // timers always run when no previous runs
        assert!(break_timer.run(&mut || {}));
        for _ in 0..100 {
            // timer will not run as 10 secs has not passed
            // do run will return false
            assert!(!break_timer.run(&mut || {}));
        }
        assert_eq!(break_timer.total_calls(), &1);
    }

    #[test]
    fn test_run_wait() {
        let mut break_timer = ThrottleTimer::new(Duration::from_nanos(10_u64), &"Break");

        break_timer.run_wait(&mut || {});
        break_timer.run_wait(&mut || {});
        break_timer.run_wait(&mut || {});
        assert_eq!(break_timer.total_calls(), &3);
    }

    #[test]
    fn test_with_delay() {
        let mut snack_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_secs(1_u64), &"Snack");
        let run_flag = snack_timer.run(&mut || {});

        // timers always run when no previous runs
        assert!(run_flag);

        let run_flag2 = snack_timer.run_with_msg(&mut || {});

        // run flag false as no time has passed
        assert_eq!(run_flag2, false);

        thread::sleep(snack_timer.max_frequency);
        assert!(snack_timer.run(&mut || {}));
        thread::sleep(Duration::from_millis(100_u64));
        assert!(!snack_timer.run(&mut || {}));
        thread::sleep(Duration::from_secs(1_u64));
        assert!(snack_timer.run(&mut || {}));
    }
}
