//! Simple library with a single struct to time events and event stats
//! Library has no dependencies
//!
//! `ThrottleTimer` struct is created with a frequency
//! Calling do_run will check the last call time. If below frequency will return false
//! If frequency duration has passed since the last call will update
//! the last called time and return true
//!
struct ThrottleTimer {
    maybe_last_called_time: Option<Instant>,
    total_calls: usize,
    created_date: SystemTime,
    frequency: &'static Duration,
    event_name: &'static str,
}

///
/// # Example
/// ```
/// use std::time::Duration;
///
/// let mut break_timer: ThrottleTimer = ThrottleTimer::new(&Duration::from_secs(1_u64), &"Break");
/// let do_break_flag = break_timer.do_run();
///
/// // Timers always run when no previous runs
/// assert!(do_break_flag == true);
/// if do_break_flag {
/// 	println!("timer do run flag is set to true")
/// }
///
/// // Run flag false as no time has passed
/// assert!(break_timer.do_run() == false);
/// ```
impl ThrottleTimer {
    pub const fn new(frequency: &'static std::time::Duration, event_name: &'static str) -> Self {
        Self {
            maybe_last_called_time: None,
            frequency,
            event_name,
            total_calls: 0,
            created_date: SystemTime::now(),
        }
    }
    pub const fn frequency(&self) -> &'static Duration {
        self.frequency
    }
    pub const fn created_date(&self) -> SystemTime {
        self.created_date
    }
    pub fn wait_time(&self) -> Duration {
        Instant::now().duration_since(self.maybe_last_called_time.unwrap()) - *self.frequency
    }

    // Calling do_run will check the last call time. If below frequency will return false
    // If frequency duration has passed since the last call will update
    // the last called time and return true
    pub fn do_run(&mut self) -> bool {
        let now = Instant::now();
        let do_run_flag: bool = match self.maybe_last_called_time {
            None => true,
            Some(last_time) => now.duration_since(last_time) >= *self.frequency,
        };

        if do_run_flag {
            self.maybe_last_called_time = Some(now);
            self.total_calls += 1;
        }
        do_run_flag
    }

    // Same as do_run but will print a message if throttled
    pub fn do_run_with_msg(&mut self) -> bool {
        let do_run_flag: bool = self.do_run();
        if !do_run_flag {
            println!(
                "{} throttled, last time {:?}, next event possible in {:?}",
                self.event_name,
                Instant::now().duration_since(self.maybe_last_called_time.unwrap()),
                self.wait_time()
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
    fn test() {
        let mut break_timer: ThrottleTimer =
            ThrottleTimer::new(&Duration::from_mins(45_u64), &"Break");
        let do_run_flag = break_timer.do_run();

        // timers always run when no previous runs
        assert!(do_run_flag == true);
        if do_run_flag {
            println!("timer do run flag is set to true")
        }
    }

    #[test]
    fn test() {
        let mut snack_timer: ThrottleTimer =
            ThrottleTimer::new(&Duration::from_secs(1_u64), &"Snack");
        let do_run_flag = snack_timer.do_run();
        assert!(do_run_flag == true); // timers always run when no previous runs
        if do_run_flag {
            println!("timer do run flag is set to true")
        }
        let do_run_flag2 = snack_timer.do_run_with_msg(&"my event");
        assert_eq!(do_run_flag2, false); // run flag false as no time has passed
        if !do_run_flag2 {
            println!("timer flag returned execute is set to false. Execution too soon")
        }
        thread::sleep(*snack_timer.frequency);
        assert!(snack_timer.do_run() == true);
        thread::sleep(Duration::from_millis(100_u64));
        assert!(snack_timer.do_run() == false);
        thread::sleep(Duration::from_secs(1_u64));
        assert!(snack_timer.do_run() == true);
    }
}
