use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Stats {
    visits: Cell<u32>,
    log: RefCell<Vec<String>>,
}

impl Stats {
    /// Creates a new Stats instance with zero visits and an empty log
    ///
    /// # Returns
    /// A new `Stats` instance initialized with default values
    pub fn new() -> Self {
        Stats {
            visits: Cell::new(0),
            log: RefCell::new(Vec::new()),
        }
    }

    /// Increments the visit counter and adds a log entry
    ///
    /// Records a new visit by incrementing the counter and adding a formatted
    /// log message with the current visit count
    pub fn add_visit(&self, string: &str) {
        self.visits.set(self.visits.get() + 1);
        self.log.borrow_mut().push(format!("thread: {}, visit: {}", string, self.visits.get()));
    }

    /// Returns the current number of visits
    ///
    /// # Returns
    /// The total number of visits recorded
    pub fn visits(&self) -> u32 {
        self.visits.get()
    }

    /// Returns a clone of the current log entries
    ///
    /// # Returns
    /// A vector containing all recorded log messages
    pub fn log(&self) -> Vec<String> {
        self.log.borrow().clone()
    }

    /// Clears all entries from the log
    ///
    /// Removes all recorded log messages while preserving the visit counter
    pub fn clear_log(&self) {
        self.log.borrow_mut().clear();
    }
}

pub fn try_interior_mutability() {
    let stats = Stats {
        visits: Cell::new(0),
        log: RefCell::new(Vec::new()),
    };

    stats.add_visit("test");
    stats.add_visit("test");
    stats.add_visit("test");

    println!("stats: {:?}", stats);
    println!("log: {:?}", stats.log());
    println!("visits: {}", stats.visits());
    println!("clear log");
    stats.clear_log();
    println!("log: {:?}", stats.log());
}

pub fn try_threading_arc_mutex() {
    let stats = Arc::new(Mutex::new(Stats::new()));

    let mut handles = vec![];

    for _ in 0..10 {
        let stat = Arc::clone(&stats);

        let handle  = thread::spawn(move || {
            let s = stat.lock().unwrap();
            let thread_id = format!("{:?}", thread::current().id());
            let thread_number = thread_id.trim_start_matches("ThreadId(").trim_end_matches(")");
            s.add_visit(thread_number);
            s.log();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("stats: {:?}", stats);
    println!("log: {:?}", stats.lock().unwrap().log());
    println!("visits: {}", stats.lock().unwrap().visits());
}