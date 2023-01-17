use std::{fmt, thread, sync::{mpsc,Arc,Mutex}};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub struct PoolCreationError<'a> {
    error_message:  &'a str,
}

struct Worker {
    id: usize,
    thread:Option<thread::JoinHandle<()>>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self:Box<F>){
        (*self)();
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("Shutting down thread");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
 enum Message {
     NewJob(Job),
     Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError<'static>> {
        if size == 0 {
            return Err(PoolCreationError{ error_message: "Size must be greater than zero" })
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

       let mut workers = Vec::with_capacity(size);

       for id in 0..size {
           workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool{
            workers,
            sender,
        })
    }

    pub fn execute<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
} 

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worler {} got a job", id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Terminating job on worker {}", id);
                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread:Some(thread),
        }
    }
}

impl<'a> fmt::Display for PoolCreationError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}
