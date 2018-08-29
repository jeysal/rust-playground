use std::sync::*;
use std::thread;

// util

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

type Job = Box<FnBox + Send + 'static>;

enum Message {
  NewJob(Job),
  Terminate,
}

// pool

pub struct ThreadPool {
  sender: mpsc::Sender<Message>,
  workers: Vec<Worker>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0, "ThreadPool must have size > 0");

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { sender, workers }
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
    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }
    for worker in &mut self.workers {
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

// worker

struct Worker {
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let msg = receiver.lock().unwrap().recv().unwrap();
      match msg {
        Message::NewJob(job) => {
          println!("Worker {} got a job.", id);
          job.call_box();
        }
        Message::Terminate => {
          println!("Worker {} shutting down.", id);
          break;
        }
      }
    });

    Worker {
      thread: Some(thread),
    }
  }
}
