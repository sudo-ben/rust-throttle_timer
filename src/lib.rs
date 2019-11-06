#![feature(duration_float)]
//! Throttle events and record event stats with a simple library
//!
//! throttle_timer has no dependencies
//!
//! `ThrottleTimer` struct is created with a max frequency and label
//!
//! ```ThrottleTimer::new(Duration::from_secs(1_u64), &"Once every second");```
//!
//! Calling ```do_run()``` will check the last call time. If max frequency time has not passed the fn will return false.
//! If max_frequency duration has passed since the last call then the fn will return true
//!
//!
//! # Example
//! ```
//! use std::time::Duration;
//!
//! let mut break_timer: ThrottleTimer = ThrottleTimer::new(Duration::from_secs(1_u64), &"Break");
//! let do_break_flag = break_timer.do_run();
//!
//! // Timers always run when no previous runs
//! assert!(do_break_flag == true);
//! if do_break_flag {
//!     println!("timer do run flag is set to true")
//! }
//!
//! // Run flag false as no time has passed
//! assert!(break_timer.do_run() == false);
//! ```

use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;

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
///
/// let mut break_timer: ThrottleTimer = ThrottleTimer::new(Duration::from_secs(1_u64), &"Break");
/// let do_break_flag = break_timer.do_run();
///
/// // Timers always run when no previous runs
/// assert!(do_break_flag == true);
/// if do_break_flag {
///     println!("timer do run flag is set to true")
/// }
///
/// // Run flag false as no time has passed
/// assert!(break_timer.do_run() == false);
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
    pub const fn total_calls(&self) -> &usize {
        &self.total_calls
    }
    pub const fn max_frequency(&self) -> &Duration {
        &self.max_frequency
    }
    pub const fn created_date(&self) -> SystemTime {
        self.created_date
    }

    /// Prints total calls and calls/sec
    pub fn print_stats(&self) {
        match self.created_date.elapsed() {
            Ok(created_time_elapsed) => {
                println!(
                    "{} calls {}/sec, total calls {}, has been running for {:?}",
                    self.event_name,
                    created_time_elapsed.as_secs_f64() / self.total_calls as f64,
                    self.total_calls,
                    created_time_elapsed,
                );
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }

    /// Calling ```do_run()``` will check the last call time. If max frequency time has not passed the fn will return false.
    /// If max_frequency duration has passed since the last call then the fn will return true
    pub fn do_run(&mut self) -> bool {
        let now = Instant::now();
        let do_run_flag: bool = match self.maybe_last_called_time {
            None => true,
            Some(last_time) => now.duration_since(last_time) >= self.max_frequency,
        };

        if do_run_flag {
            self.maybe_last_called_time = Some(now);
            self.total_calls += 1;
        }
        do_run_flag
    }

    /// Same as do_run but will print a message if throttled
    pub fn do_run_with_msg(&mut self) -> bool {
        let do_run_flag: bool = self.do_run();
        if !do_run_flag {
            println!(
                "{} throttled, last time {:?}",
                self.event_name,
                Instant::now().duration_since(self.maybe_last_called_time.unwrap())
            );
        }
        do_run_flag
    }
}

#[cfg(test)]
mod test {
    use super::ThrottleTimer;
    use std::{thread, time::Duration};

    #[test]
    fn test_do_run() {
        let mut break_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_secs(45_000_u64), &"Break");
        let do_run_flag = break_timer.do_run();

        // timers always run when no previous runs
        assert!(do_run_flag);
        if do_run_flag {
            println!("timer do run flag is set to true")
        }
    }

    #[test]
    fn test_do_run_with_msg() {
        let mut break_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_secs(45_000_u64), &"Break");
        let do_run_flag = break_timer.do_run_with_msg();

        // timers always run when no previous runs
        assert!(do_run_flag);
        if do_run_flag {
            println!("timer do run flag is set to true")
        }
        break_timer.total_calls();
        break_timer.max_frequency();
        break_timer.created_date();
    }

    #[test]
    fn test_call_count() {
        let mut break_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_nanos(1_u64), &"Break");

        for _ in 0..100 {
            assert_eq!(break_timer.do_run(), true);
            thread::sleep(Duration::from_nanos(100_u64));
        }

        // timers always run when no previous runs
        assert_eq!(break_timer.total_calls, 100);
        break_timer.print_stats();
    }

    #[test]
    fn test_in_loop() {
        let mut break_timer = ThrottleTimer::new(Duration::from_secs(10_u64), &"Break");

        // timers always run when no previous runs
        assert!(break_timer.do_run());
        for _ in 0..100 {
            // timer will not run as 10 secs has not passed
            // do run will return false
            assert!(!break_timer.do_run());
        }
        assert_eq!(break_timer.total_calls(), &1);
    }

    #[test]
    fn test_with_delay() {
        let mut snack_timer: ThrottleTimer =
            ThrottleTimer::new(Duration::from_secs(1_u64), &"Snack");
        let do_run_flag = snack_timer.do_run();
        assert!(do_run_flag); // timers always run when no previous runs
        if do_run_flag {
            println!("timer do run flag is set to true")
        }
        let do_run_flag2 = snack_timer.do_run_with_msg();
        assert_eq!(do_run_flag2, false); // run flag false as no time has passed
        if !do_run_flag2 {
            println!("timer flag returned execute is set to false. Execution too soon")
        }

        thread::sleep(snack_timer.max_frequency);
        assert!(snack_timer.do_run());
        thread::sleep(Duration::from_millis(100_u64));
        assert!(!snack_timer.do_run());
        thread::sleep(Duration::from_secs(1_u64));
        assert!(snack_timer.do_run());
    }
}
