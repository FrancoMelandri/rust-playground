use std::time::Instant;

pub struct Stopwatch {
    name: String,
    start_time: Instant,
    pub elapsed: f64,
}

fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
    let t = t2 - t1;
    t.as_secs() as f64 * 1000. + t.subsec_nanos() as f64 / 1e6
}

impl Stopwatch {
    pub fn start(name: &str) -> Self {
        Stopwatch {
            name: name.to_string(),
            start_time: Instant::now(),
            elapsed: 0f64,
        }
    }

    pub fn stop(self) -> Stopwatch {
        Stopwatch {
            name: self.name.clone(),
            start_time: self.start_time,
            elapsed: elapsed_ms(self.start_time, Instant::now()),
        }
    }
}

impl Drop for Stopwatch {
    fn drop(&mut self) {
        println!("Drop Stopwatch {} elapsed {}", self.name, self.elapsed)
    }
}
