use std::time::{Duration, Instant};

pub fn time_span<R>(f: impl FnOnce() -> R) -> (R, Duration) {
    let start = Instant::now();
    let result = f();

    (result, Instant::now() - start)
}