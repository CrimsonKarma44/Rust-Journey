use std::{
    io::Error,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread,
};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `build` function will panic if the size is zero.
    pub fn build(size: usize) -> Result<Self, Error> {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let threads = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect::<Vec<_>>();
        Ok(ThreadPool { threads, sender })
    }


    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    thread: thread::JoinHandle<()>,
    id: usize,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        Worker {
            thread: thread::spawn(move || {
                loop {
                    let job = receiver.lock().unwrap().recv().unwrap();
                    println!("Worker {id} got job: executing");
                    job();
                    println!("Worker {id} finished job: exiting");
                }
            }),
            id,
        }
    }
}
