use std::{collections::VecDeque, sync::{Arc, Condvar, Mutex}, thread::JoinHandle};

/// A thread pool of workers that can execute tasks.
/// Use `Workers::new(amount)` to create a new group
/// of workers. Add new tasks with
/// `workers.post(task)`.
pub struct Workers<T> where T: FnOnce() {
    amount: u32,
    threads: Vec<JoinHandle<()>>,
    condition_variable: Arc<(Mutex<bool>, Condvar)>,
    tasks: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Workers<T> where T: FnOnce() + Send + 'static {
    pub fn new(amount: u32) -> Self {
        let mut workers = Workers {
            amount,
            threads: Vec::new(),
            condition_variable: Arc::new((Mutex::new(false), Condvar::new())),
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        };
        workers.wait_for_tasks();
        workers
    }

    fn wait_for_tasks(&mut self) {
        for _ in 0..self.amount {
            let tasks = self.tasks.clone();
            let condvar = self.condition_variable.clone();
            self.threads.push(std::thread::spawn(move || {
                loop {
                    let (lock, cvar) = &*condvar;
                    {
                        let mut ready = lock.lock().unwrap();
                        while !*ready {
                            ready = cvar.wait(ready).unwrap();
                        }

                    }
                    let mut task: Option<T> = None;
                    {
                        let mut tasks = tasks.lock().unwrap();
                        if !tasks.is_empty() {
                            task = tasks.pop_front();
                        }
                    }
                    if let Some(func) = task {
                        func();
                    }
                    *lock.lock().unwrap() = false
                }
            }));
        }
    }

    pub fn post(&mut self, task: T) {
        self.tasks.lock().unwrap().push_back(task);
        let (lock, cvar) = &*self.condition_variable;
        *lock.lock().unwrap() = true;
        cvar.notify_all();
    }
}
