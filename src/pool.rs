use std::{
    sync::{Arc, Mutex, mpsc::{self, Receiver}},
    thread
};

// Defining a closure trait object, that will be shared through the channel and initialized by execute.
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    relay: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize, reciever: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = reciever.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {id} activated");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected");
                        break;
                    }
                }
            }
        });
        Worker { id, thread }
    }
}

impl ThreadPool {
    pub fn new(num: usize) -> ThreadPool {
        assert!(num > 0);

        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut relay = Vec::with_capacity(num);
        for id in 0..num {
            relay.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { relay, sender: Some(sender) } 
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Send + FnOnce() + 'static,
        {
            let job = Box::new(f);
            self.sender
            .as_ref()
            .unwrap()
            .send(job).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // we need to drop the sender to stop the infinite loop in the thread closure.

        for worker in self.relay.drain(..) { // using self.drain() to deal with the ownership of threads issue.
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap(); // calling join alone won't shut down threads, since they loop forever.
        }
    }
}
