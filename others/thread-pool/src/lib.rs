use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

type Task = Option<Box<dyn FnOnce() + Send>>;

pub struct ThreadPool {
    n_threads: usize,
    is_active: bool,
    join_handles: Vec<JoinHandle<()>>,
    queue_sender: SyncSender<Task>,
    queue_receiver: Arc<Mutex<Receiver<Task>>>,
}

impl ThreadPool {
    pub fn new(queue_size: usize, n_threads: usize) -> Option<Self> {
        if (queue_size == 0) || (n_threads == 0) {
            return None;
        }

        let (s, r) = sync_channel::<Task>(queue_size);

        Some(ThreadPool {
            n_threads,
            is_active: false,
            join_handles: vec![],
            queue_sender: s,
            queue_receiver: Arc::new(Mutex::new(r)),
        })
    }

    pub fn start(&mut self) -> bool {
        if self.is_active {
            return false;
        }
        self.is_active = true;

        for _ in 0..self.n_threads {
            let receiver = self.queue_receiver.clone();

            let handle = thread::spawn(move || loop {
                let result = receiver.lock().unwrap().recv();

                match result {
                    Ok(opt_task) => match opt_task {
                        Some(task) => task(),
                        None => break,
                    },
                    Err(_) => break,
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

        for _ in 0..self.n_threads {
            self.queue_sender.send(None).unwrap();
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

        self.queue_sender.send(Some(Box::new(task))).unwrap();
        true
    }
}
