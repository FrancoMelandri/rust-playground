mod stop_watch_tests {
    use std::{thread::sleep, time::Duration};

    use helloworld::Stopwatch;

    #[test]
    fn test_times_is_wotking() {
        let mut times: Stopwatch = Stopwatch::start("sw");
        assert_eq!(times.elapsed, 0f64);
        sleep(Duration::from_secs(1));
        times = times.stop();
        assert_eq!(times.elapsed > 1000f64 && times.elapsed < 1001f64, true);
    }
}
