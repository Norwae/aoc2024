use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};


pub fn time_span<R>(f: impl FnOnce() -> R) -> (R, Duration) {
    let start = Instant::now();
    let result = f();

    (result, Instant::now() - start)
}

static WORK: AtomicU64 = AtomicU64::new(0);

pub fn work_duration() -> Duration {
    let nanos = WORK.load(Ordering::Relaxed);
    Duration::from_nanos(nanos)
}

pub fn work(f: impl FnOnce()) {
    let (_, duration) = time_span(f);
    let nanos = duration.as_nanos();
    WORK.fetch_add(nanos as u64, Ordering::AcqRel);
}