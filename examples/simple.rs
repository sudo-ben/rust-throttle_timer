use std::time::Duration;
use throttle_timer::ThrottleTimer;

fn main() {
    let mut break_timer = ThrottleTimer::new(Duration::from_secs(10_u64), &"Break");
    let mut val = 0_u8;
    // timers always run when no previous runs
    assert!(break_timer.do_run(&mut || val += 1));
    for _ in 0..100 {
        // timer will not run as 10 secs has not passed
        // do run will return false
        assert!(!break_timer.do_run(&mut || val += 1));
    }
    break_timer.print_stats();
    assert_eq!(break_timer.total_calls(), &1);
    assert_eq!(val, 1_u8);
}
