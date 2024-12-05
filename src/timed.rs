use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use lazy_static::lazy_static;

pub const ALL_WORK: &'static str = "ALL_WORK";

pub fn time_span<R>(f: impl FnOnce() -> R) -> (R, Duration) {
    let start = Instant::now();
    let result = f();

    (result, Instant::now() - start)
}

lazy_static! {
    static ref BIN_DURATIONS: Mutex<HashMap<&'static str, Duration>> = Mutex::new(HashMap::new());
}

pub fn time_span_to_bin<R>(bin: &'static str, f: impl FnOnce() -> R) -> R {
    let (result, time) = time_span(f);

    let mut lock = BIN_DURATIONS.lock().unwrap();
    let mut total_time = lock.entry(bin).or_insert(Duration::ZERO);
    *total_time += time;

    result
}

pub fn bin_duration(bin: &str) -> Duration {
    BIN_DURATIONS.lock().unwrap().get(bin).cloned().unwrap_or_default()
}