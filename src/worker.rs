use std::io::Write;
use std::sync::mpsc::{channel, sync_channel};
use lazy_static::lazy_static;
use threadpool::ThreadPool;
use crate::ui::UIOutput;
lazy_static! {
    static ref THREADPOOL: ThreadPool = threadpool::Builder::new().build();
}

pub fn run_on_worker(function: impl FnOnce() + Send + 'static) {
    (*THREADPOOL).execute(function)
}

pub fn parallelize<F, R, I>(candidates: I) -> Vec<R>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + 'static,
    I: IntoIterator<Item=F>,
{
    let (send, recv) = channel();
    let mut count = 0;
    for (n, f) in candidates.into_iter().enumerate() {
        let send = send.clone();
        run_on_worker(move || {
            let result = f();
            let tagged_result = (n, result);
            send.send(tagged_result).expect("Sending okay");
        });
        count += 1;
    }
    let mut tagged_answers = (0..count).map(|_| recv.recv().expect("Got result")).collect::<Vec<_>>();
    tagged_answers.sort_by_key(|(fst, _)| *fst);
    tagged_answers.into_iter().map(|(_, snd)| snd).collect()
}

pub fn race<F, R, I, O, W>(out: &mut O, candidates: I) -> R
where
    W: Write,
    O: UIOutput<W>,
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
    I: IntoIterator<Item=F>
{
    let available_threads = THREADPOOL.max_count() - 1;
    let mut n = 0;
    let candidates = candidates.into_iter();
    let (send, receive) = sync_channel(1);
    for candidate in candidates {
        if n > available_threads {
            out.critical(format_args!("Would attempt to race more than {} solutions, which would force scheduling (=not even participating in the race). Discarding tail", available_threads));
            break;
        }

        n += 1;
        let send = send.clone();
        run_on_worker(move || {
            let result = candidate();
            _ = send.try_send(result)
        })
    }

    receive.recv().expect("Received an answer")
}

#[cfg(test)]
mod test {
    use std::fmt::Arguments;
    use std::thread::sleep;
    use std::time::Duration;
    use rand::{RngCore, thread_rng};
    use crate::ui::UIOutput;
    use crate::worker::{parallelize, race};

    struct NoUI;

    impl UIOutput<Vec<u8>> for NoUI {
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
        assert_eq!(1, race(&mut NoUI, solvers))
    }

    #[test]
    fn discards_race_tail() {
        let tasks = (0u64..100).rev().map(|n|{
            let delay = Duration::from_millis(n);
            move ||{
                sleep(delay);
                n
            }
        }).collect::<Vec<_>>();
        let fastest = race(&mut NoUI, tasks);
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