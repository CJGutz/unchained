use std::{
    collections::VecDeque,
    fmt::Debug,
    sync::{Arc, Condvar, Mutex},
    thread,
};

/// A thread pool of workers that can execute tasks.
/// Use `Workers::new(amount)` to create a new group
/// of workers. Add new tasks with
/// `workers.post(task)`.
pub struct Workers<T, U>
where
    T: FnOnce() -> Result<(), U>,
{
    amount: u32,
    threads: Vec<thread::JoinHandle<()>>,
    condition_variable: Arc<(Mutex<bool>, Condvar)>,
    tasks: Arc<Mutex<VecDeque<T>>>,
}

impl<T, U> Workers<T, U>
where
    T: FnOnce() -> Result<(), U> + Send + 'static,
    U: Debug,
{
    /// Create a new group of workers.
    /// Also starts waiting for tasks.
    /// Example:
    /// ```
    /// use unchained_web::workers::Workers;
    /// let mut workers = Workers::new(4);
    /// workers.post(|| {
    ///    println!("Hello from a worker!");
    ///    Ok::<(), String>(())
    /// });
    /// ```
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

    /// Add a new task to the worker pool.
    pub fn post(&self, task: T) {
        self.tasks.lock().unwrap().push_back(task);
        let (lock, cvar) = &*self.condition_variable;
        *lock.lock().unwrap() = true;
        cvar.notify_all();
    }

    pub fn start_thread(&mut self) {
        let tasks = self.tasks.clone();
        let condvar = self.condition_variable.clone();
        self.threads.push(thread::spawn(move || loop {
            let (lock, cvar) = &*condvar;
            {
                let mut ready = lock.lock().unwrap();
                while !*ready {
                    ready = cvar.wait(ready).unwrap();
                }
            }
            #[allow(unused_assignments)] // We want the tasks lock out of scope
            let mut task: Option<T> = None;
            {
                let mut tasks = tasks.lock().unwrap();
                task = tasks.pop_front();
            }
            if let Some(func) = task {
                let result = func();
                if let Err(e) = result {
                    eprintln!("Error in worker: {:?}\nContinuing work ...\n", e);
                }
            }
            *lock.lock().unwrap() = false
        }));
    }

    /// Run the workers and be ready to execute tasks.
    fn wait_for_tasks(&mut self) {
        for _ in 0..self.amount {
            self.start_thread();
        }
    }
}
