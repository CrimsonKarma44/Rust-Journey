use std::{
    io::Error,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `build` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<Self, Error> {
        assert!(size > 0);
        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let threads = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect::<Vec<_>>();
        Ok(ThreadPool {
            workers: threads,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    id: usize,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = Some(thread::spawn(move || {
            loop {
                let message = receiver
                    .lock()
                    .expect("failed to receive lock on receiver")
                    .recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got job: executing");
                        job();
                        println!("Worker {id} finished job: exiting");
                    }

                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        }));
        Worker { thread: thread, id }
    }
}
