use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


// Transitioning to a ThreadPool had a few complexities: 
// Ultimately, we will set up a channel to pass an instance of the 
// connection handler (as a closure) from the ThreadPool’s execute() method to a Worker thread. 
// One complexity is that there can only be one receiver on the channel: 
// We needed to set up a mutex for sharing the receiver in a safe way across the multiple Worker threads. 
// The use of smart pointers (Arc, Box) for passing references around is very useful 
// and the explicitness of it is beneficial, but the requisite, idiomatic syntax takes some getting used to.
// BTW, note the use of the loop construct inside the thread’s closure… very nice

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // we need to recast the receiver into something that can be shared by multiple worker threads:
        // a Mutex inside of an Arc
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // set up the workers
            workers.push(Worker::new(id+5000, Arc::clone(&receiver))); // each clone of the Arc bumps the reference count
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) 
        where 
            F: FnOnce() + Send + 'static, 
        {
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("drop()... Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("drop()... Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker with id={}", worker.id);

            if let Some(thread) = worker.inner_thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
        id: usize,
        inner_thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Create a new ThreadWorker
    /// 
    /// The id is the identifier for the inner thread
    /// The receiver is the receive end of our channel
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker  {
        let inner_thread = std::thread::spawn(move || 
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job to execute. Go!!!", id);

                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate. Stop!!!", id);

                        break;
                    }
                } 

            });    

        Worker { id, inner_thread: Some(inner_thread) }
    }
}
