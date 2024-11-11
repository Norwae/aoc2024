use lazy_static::lazy_static;
use threadpool::ThreadPool;
lazy_static! {
    static ref THREADPOOL: ThreadPool = threadpool::Builder::new().build();
}

pub fn  run_on_worker<F: FnOnce() + Send + 'static>(function: F) {
    (*THREADPOOL).execute(function)
}