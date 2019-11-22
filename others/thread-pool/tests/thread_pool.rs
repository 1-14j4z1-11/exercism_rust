use std::cell::Cell;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use thread_pool::ThreadPool;

#[test]
fn test_constructor_illegal_argument_first() {
    assert_eq!(ThreadPool::new(0, 1).is_none(), true);
}

#[test]
fn test_constructor_illegal_argument_second() {
    assert_eq!(ThreadPool::new(1, 0).is_none(), true);
}

#[test]
fn test_start_and_stop() {
    let mut tp = ThreadPool::new(1, 1).unwrap();
    assert_eq!(tp.start(), true);
    assert_eq!(tp.stop(), true);
}

#[test]
fn test_stop_before_start() {
    let mut tp = ThreadPool::new(1, 1).unwrap();
    assert_eq!(tp.stop(), false);
}

#[test]
fn test_restart_without_stop() {
    let mut tp = ThreadPool::new(1, 1).unwrap();
    assert_eq!(tp.start(), true);
    assert_eq!(tp.start(), false);
}

#[test]
fn test_dispatch_before_start() {
    let mut tp = ThreadPool::new(1, 1).unwrap();
    assert_eq!(tp.dispatch(|| {}), false);
}

#[test]
fn test_simple_dispatch() {
    let mut tp = ThreadPool::new(1, 1).unwrap();
    tp.start();

    let is_called = Arc::new(Mutex::new(Cell::new(false)));

    {
        let is_called = is_called.clone();

        tp.dispatch(move || {
            let c = is_called.lock().unwrap();
            c.set(true);
        });
    }

    wait_until(|| is_called.lock().unwrap().get(), 100);
    tp.stop();
}

#[test]
fn test_simple_repeated_dispatch() {
    let mut tp = ThreadPool::new(1, 1).unwrap();
    tp.start();

    let n_tasks = 10;
    let counter = Arc::new(Mutex::new(Cell::new(0)));

    for _ in 0..n_tasks {
        let counter = counter.clone();

        tp.dispatch(move || {
            let c = counter.lock().unwrap();
            c.set(c.get() + 1);
        });
    }

    wait_until(|| counter.lock().unwrap().get() == n_tasks, 100);
    tp.stop();
}

#[test]
fn test_complex_repeated_dispatch() {
    let mut tp = ThreadPool::new(10, 10).unwrap();
    tp.start();

    let n_tasks = 1000;
    let counter = Arc::new(Mutex::new(Cell::new(0)));

    for _ in 0..n_tasks {
        let counter = counter.clone();

        tp.dispatch(move || {
            let c = counter.lock().unwrap();
            c.set(c.get() + 1);
        });
    }

    wait_until(|| counter.lock().unwrap().get() == n_tasks, 100);
    tp.stop();
}

#[test]
fn test_complex_repeated_dispatch2() {
    let mut tp = ThreadPool::new(1, 1).unwrap();
    tp.start();

    let counters = (0..10)
        .map(|_| Arc::new(Mutex::new(Cell::new(0))))
        .collect::<Vec<_>>();
    let n_loop = 100;

    for _ in 0..n_loop {
        for counter in &counters {
            let counter = counter.clone();

            tp.dispatch(move || {
                let c = counter.lock().unwrap();
                c.set(c.get() + 1);
            });
        }
    }

    for counter in &counters {
        wait_until(|| counter.lock().unwrap().get() == n_loop, 1000);
    }

    tp.stop();
}

#[test]
fn test_latch_simple_dispatch() {
    let n_threads = 10;
    let mut tp = ThreadPool::new(10, n_threads).unwrap();
    tp.start();

    let counter = Arc::new(Mutex::new(Cell::new(0)));

    for _ in 0..n_threads {
        let counter = counter.clone();
        tp.dispatch(move || {
            {
                let c = counter.lock().unwrap();
                c.set(c.get() + 1);
            }

            wait_until(move || counter.lock().unwrap().get() >= n_threads, 1000);
        });
    }

    let counter = counter.clone();
    wait_until(move || counter.lock().unwrap().get() >= n_threads, 1000);
    tp.stop();
}

#[test]
fn test_latch_complex_dispatch() {
    let n_threads = 10;
    let mut tp = ThreadPool::new(10, n_threads).unwrap();
    tp.start();

    let n_tasks = 10;
    let counters = (0..n_tasks)
        .map(|_| Arc::new(Mutex::new(Cell::new(0))))
        .collect::<Vec<_>>();

    for counter in &counters {
        for _ in 0..n_threads {
            let counter = counter.clone();
            tp.dispatch(move || {
                {
                    let c = counter.lock().unwrap();
                    c.set(c.get() + 1);
                }

                wait_until(move || counter.lock().unwrap().get() >= n_threads, 1000);
            });
        }
    }

    for counter in &counters {
        wait_until(move || counter.lock().unwrap().get() >= n_threads, 1000);
    }

    tp.stop();
}

#[test]
fn test_number_of_threads() {
    let n_threads = 10;
    let thread_id_set = Arc::new(Mutex::new(HashSet::new()));
    let mut tp = ThreadPool::new(10, n_threads).unwrap();
    tp.start();

    for _ in 0..n_threads * 3 {
        let id_set = thread_id_set.clone();
        tp.dispatch(move || {
            let id = thread::current().id();
            id_set.lock().unwrap().insert(id);

            thread::sleep(Duration::from_millis(100));
        });
    }

    tp.stop();
    assert_eq!(thread_id_set.lock().unwrap().len(), n_threads);
}

fn wait_until<F>(condition: F, timeout_ms: u64)
where
    F: Fn() -> bool,
{
    let delay_ms = 10;
    let mut total_wait_ms = 0;

    while !condition() {
        thread::sleep(Duration::from_millis(delay_ms));
        total_wait_ms += delay_ms;

        if total_wait_ms >= timeout_ms {
            assert!(false, "wait_until() : time out");
        }
    }
}
