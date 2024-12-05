use std::sync::mpsc::{channel, sync_channel};
use lazy_static::lazy_static;
use threadpool::ThreadPool;
use crate::timed::{time_span_to_bin, ALL_WORK};

lazy_static! {
    static ref THREADPOOL: ThreadPool = threadpool::Builder::new().build();
}

pub fn run_on_worker(function: impl FnOnce() + Send + 'static) {
    (*THREADPOOL).execute(||time_span_to_bin(ALL_WORK,function))
}

pub fn parallelize_ordered<F, R, I>(tasks: I) -> Vec<R>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + 'static,
    I: IntoIterator<Item=F>,
{
    let it = tasks.into_iter().enumerate().map(|(n, f0)|{
        move ||{
            let result = f0();
            (n, result)
        }
    });
    let mut tagged_results = parallelize(it);
    tagged_results.sort_by_key(|(f, _)|*f);
    tagged_results.into_iter().map(|(_, s)|s).collect()
}

pub fn parallelize<F, R, I>(tasks: I) -> Vec<R>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + 'static,
    I: IntoIterator<Item=F>,
{
    let (send, recv) = channel();
    for f in tasks.into_iter(){
        let send = send.clone();
        run_on_worker(move || {
            let result = f();
            send.send(result).unwrap();
        });
    }
    drop(send);

    let mut tagged_answers = Vec::new();
    while let Ok(tpl) = recv.recv() {
        tagged_answers.push(tpl)
    }
    tagged_answers
}

pub fn race<F, R, I>(candidates: I) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
    I: IntoIterator<Item=F>,
{
    let available_threads = THREADPOOL.max_count() - 1;
    let mut n = 0;
    let (send, receive) = sync_channel(1);
    for candidate in candidates {
        if n > available_threads {
            break;
        }

        n += 1;
        let send = send.clone();
        run_on_worker(move || {
            let result = candidate();
            _ = send.try_send(result);
        })
    }

    receive.recv().expect("Received an answer")
}


pub fn warm_up() {
    race((0..1000).map(|n| {
        move ||{
            n
        }
    }).collect::<Vec<_>>());
}

#[cfg(test)]
mod test {
    use std::fmt::{Arguments};
    use std::io::Write;
    use std::thread::sleep;
    use std::time::Duration;
    use rand::{RngCore, thread_rng};
    use crate::ui::UIWrite;
    use crate::worker::{parallelize, race};

    struct NoUI;

    impl Write for NoUI {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    impl UIWrite for NoUI {
        fn info(&mut self, _fmt: Arguments<'_>) {}

        fn critical(&mut self, _fmt: Arguments<'_>) {}

        fn result(&mut self, _fmt: Arguments<'_>) {}
    }

    #[test]
    fn fast_solve_on_race() {
        let solvers = vec![|| {
            sleep(Duration::from_millis(1000));
            0
        }, || {
            1
        }];
        assert_eq!(1, race(solvers))
    }

    #[test]
    fn discards_race_tail() {
        let tasks = (0u64..100).rev().map(|n| {
            let delay = Duration::from_millis(n);
            move || {
                sleep(delay);
                n
            }
        }).collect::<Vec<_>>();
        let fastest = race(tasks);
        assert!(fastest > 50)
    }

    #[test]
    fn retains_order_in_parallelize() {
        let tasks = (0usize..50).map(|n| {
            let millis = 50 + (thread_rng().next_u32() % 200) as u64;
            move || {
                sleep(Duration::from_millis(millis));
                n
            }
        }).collect::<Vec<_>>();
        let result = parallelize(tasks);
        assert_eq!(Vec::from_iter(0..50usize), result)
    }
}