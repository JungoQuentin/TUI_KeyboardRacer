use std::time::Instant;

pub fn delta_time(start: Instant) -> f64 {
    let end = Instant::now();
    let delta = end.duration_since(start).as_secs_f64();
    delta
}
