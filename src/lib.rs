use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type JobRx = Arc<Mutex<mpsc::Receiver<Job>>>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: JobRx) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job", id);

                job();
            }),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(job)).unwrap()
    }
}
