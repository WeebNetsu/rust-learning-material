use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Worker {
    id: usize,
    // Option allows for optional values (value or None)
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker {
            id,
            // we use Some because thread is an optional value (Some - just like in Nim)
            thread: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; Executing.", id);

                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            })),
        }
    }
}

impl ThreadPool {
    // /// will create code documentation for the function
    // it can be styled with markdown
    // you can view docs with "cargo doc --open"
    // or by hovering over the function in vs code

    /// Create a new ThreadPool
    ///
    /// *size* - Number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is 0
    pub fn new(size: usize) -> ThreadPool {
        // usize: unsigned integer depending on the architecture
        // make sure that we enter a valid size
        // we can use the assert macro to do this
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // arc is a smart pointer that will allow us to
        // share receivers between worker instances
        let receiver = Arc::new(Mutex::new(receiver));

        // unlike Vec::new, Vec::with_capacity will preallocate
        // enough space for the given size, instead of resizing
        // itself as we add elements
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // we create a new thread and add it to the vector
            // we don't start the thread yet
            // threads.push(thread::spawn(|| {}));
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    // fnOnce is a function that is only executed once
    // it is used to initialize a static variable,
    // so it will not be executed multiple times
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // we have 2 loops, mainly because if one thread is busy and we tried
        // to call .join on it, then it might give the job to another thread...
        // in this case we're first giving the jobs, then trying to close the threads
        for _ in &self.workers {
            // send stop message to all workers
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        // we use &mut because we're modifying this self mutable reference
        for worker in &mut self.workers {
            // stop all workers
            println!("Shutting down worker {}", worker.id);

            // .take -> this will take the value out of worker.thread and make it
            // equal to None
            // this if statement declares a new variable, but also checks if
            // thread is not empty
            if let Some(thread) = worker.thread.take() {
                // we do not want to catch this unwrap, since if we're shutting down
                // the server and cleaning up the threads, if they fail to close out
                // panicing would be fine
                thread.join().unwrap();
            }
        }
    }
}
