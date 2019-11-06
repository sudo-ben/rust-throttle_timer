use std::time::Duration;
use throttle_timer::ThrottleTimer;

fn main() {
    let mut break_timer = ThrottleTimer::new(Duration::from_secs(10_u64), &"Break");

    // timers always run when no previous runs
    assert!(break_timer.do_run());
    for _ in 0..100 {
        // timer will not run as 10 secs has not passed
        // do run will return false
        assert!(!break_timer.do_run());
    }
    break_timer.print_stats();
    assert_eq!(break_timer.total_calls(), &1);
}
