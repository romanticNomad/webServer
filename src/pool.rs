use std::{
    sync::{Arc, Mutex, mpsc::{self, Receiver}},
    thread
};

// Defining a closure that will be shared through the channel and initialized by execute.
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    relay: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize, reciever: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            let job = reciever.lock().unwrap().recv().unwrap();
            println!("{id} online");
            job()
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

        ThreadPool { relay, sender } 
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Send + FnOnce() + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}
