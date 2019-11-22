use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

type TaskQueue = Arc<Mutex<VecDeque<Option<Box<dyn FnOnce() + Send>>>>>;

pub struct ThreadPool {
    queue_size: usize,
    n_threads: usize,
    is_active: bool,
    join_handles: Vec<JoinHandle<()>>,
    queue: TaskQueue,
}

impl ThreadPool {
    pub fn new(queue_size: usize, n_threads: usize) -> Option<Self> {
        if (queue_size == 0) || (n_threads == 0) {
            return None;
        }

        Some(ThreadPool {
            queue_size,
            n_threads,
            is_active: false,
            join_handles: vec![],
            queue: Arc::new(Mutex::new(VecDeque::new())),
        })
    }

    pub fn start(&mut self) -> bool {
        if self.is_active {
            return false;
        }
        self.is_active = true;

        for _ in 0..self.n_threads {
            let queue = self.queue.clone();
            let handle = thread::spawn(move || loop {
                let item = {
                    let mut queue = queue.lock().unwrap();
                    queue.pop_front()
                };
                match item {
                    Some(opt_task) => match opt_task {
                        Some(task) => task(),
                        None => break,
                    },
                    None => thread::sleep(Duration::from_millis(100)),
                }
            });

            self.join_handles.push(handle);
        }

        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.is_active {
            return false;
        }

        {
            let mut queue = self.queue.lock().unwrap();

            for _ in 0..self.n_threads {
                queue.push_back(None);
            }
        }

        while !self.join_handles.is_empty() {
            let handle = self.join_handles.pop().unwrap();
            handle.join().unwrap();
        }

        self.is_active = false;

        true
    }

    pub fn dispatch<F>(&mut self, task: F) -> bool
    where
        F: FnOnce() + Send + 'static,
    {
        if !self.is_active {
            return false;
        }

        let mut queue = self.queue.lock().unwrap();
        queue.push_back(Some(Box::new(task)));

        true
    }
}
