use std::{net::TcpStream, sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

pub struct Worker { 
    id : usize,
    thread : JoinHandle<()>,    
}

impl Worker {
    pub fn new(id: usize, receiver : Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move ||{
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("worker : {} is executing ", id);
                job();
            }
        });
        Worker{id , thread}
    }
    
}

pub struct ThreadPool { 
    workers : Vec<Worker>,
    sender : mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool { 
    pub fn new(size : usize) -> Self {
        let mut workers = Vec::with_capacity(size);
        let (sender , receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            let worker = Worker::new(i, Arc::clone(&receiver));
            workers.push(worker);
        }
        ThreadPool{workers, sender}
    }
    pub fn execute<F, Args>(&self, f:F, args: Args) 
    where
        F : FnOnce(Args) + Send + 'static,
        Args : Send + 'static
    {
        let job = Box::new(move || f(args) );
        self.sender.send(job).unwrap();
    }
}