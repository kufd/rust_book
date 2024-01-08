use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Err(error) = self.sender.as_ref().unwrap().send(job) {
            eprintln!("Cannot send job. Error: {}", error);
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                if let Err(error) = thread.join() {
                    eprintln!("Joining thread error: {:?}", error);
                }
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = match receiver.lock() {
                Ok(message) => message.recv(),
                Err(error) => {
                    eprintln!("Worker cannot receive job. Error: {}", error);
                    continue;
                },
            };

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, mpsc, Mutex};
    use crate::{ThreadPool, Worker};

    #[test]
    fn create_worker() {
        let (_, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let worker = Worker::new(10, Arc::clone(&receiver));

        assert_eq!(10, worker.id);
        assert_eq!(true, worker.thread.is_some());
    }

    #[test]
    #[should_panic(expected = "assertion failed: size > 0")]
    fn invalid_size_of_thread_pool() {
        ThreadPool::new(0);
    }

    #[test]
    fn thread_pool_execute_job() {
        let numbers = Arc::new(Mutex::new(Box::new(Vec::new())));

        {
            let thread_pool = ThreadPool::new(5);
            for _ in 0..10 {
                let numbers_holder = Arc::clone(&numbers);
                thread_pool.execute(move || {
                    for j in 0..100 {
                        numbers_holder.lock().unwrap().push(j);
                    }
                });
            }
        }

        let mut numbers_vector = numbers.lock().unwrap();
        numbers_vector.sort();

        assert_eq!(1000, numbers_vector.len());
        assert_eq!(vec![0,0,0,0,0,0,0,0,0,0], numbers_vector[0..10]);
        assert_eq!(vec![99,99,99,99,99,99,99,99,99,99], numbers_vector[990..]);
    }
}