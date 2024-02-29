use std::sync::{mpsc, Arc, Mutex};
use std::thread;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Box<dyn FnOnce() + Send + 'static>>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool {
    pub fn new(threads: usize) -> Result<ThreadPool, PoolCreationError> {
        if threads == 0 {
            return Err(PoolCreationError::ZeroThreads);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(threads);

        for id in 0..threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.sender.send(job).unwrap();
    }
}

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroThreads,
}

fn main() {
    let thread_pool = ThreadPool::new(5).expect("failed to create pool");
    let mut y = 10;
    for i in 0..8 {
        thread_pool.spawn(move || {
            // This is the work that the thread will execute.
            println!("Executing job {}", i);
            // Simulate some work with a sleep.
            std::thread::sleep(std::time::Duration::from_secs(1));
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(10));

}
