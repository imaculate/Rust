use std::thread::JoinHandle;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

struct Worker
{
    id: usize,
    thread: Option<JoinHandle<()>>
}

enum Message
{
    NewJob(Job),
    Terminate
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker
    {
        let thread = std::thread::spawn(move || {
            //why not use while loop, makes it essentially serial
            loop
            {
                let job = receiver.lock().unwrap().recv().unwrap();

                match job
                {
                    Message::NewJob(job) => 
                    {
                        println!("Worker {} received job; executing", id);
                        job();
                    },
                    Message::Terminate => 
                    {
                        println!("Worker {} received job terminate message", id);
                        break;
                    }

                }

                
            }});

        Worker
        {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool
{
    /// Create a new pool thread
    /// 
    /// Size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// Panics if the size is zero
    pub fn new(size : usize) -> ThreadPool
    {
        assert!(size > 0);

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size
        {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        println!("Sending terminate message to all workers");
        for _ in &self.workers
        {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        println!("Shutting down all workers");
        for worker in &mut self.workers
        {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take()
            {
                thread.join().unwrap();
            }
        }
    }
}