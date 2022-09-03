use std::{
    collections::BTreeMap,
    ops::Add,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use practice::trie::HashTrie;

// Timer accuracy, scalability
// Cancellation
// Concurrent execution

#[derive(Clone)]
struct ScheduledExecutor {
    tasks: Arc<Mutex<BTreeMap<Instant, Task>>>,
}

impl ScheduledExecutor {
    pub fn new(tick_speed: Duration) -> Self {
        let tasks = Arc::new(Mutex::new(BTreeMap::new()));
        let tasks_cloned: Arc<Mutex<BTreeMap<Instant, Task>>> = tasks.clone();
        thread::spawn(move || loop {
            let time = Instant::now();
            let mut expired: Vec<Task> = vec![];
            {
                let mut tasks = tasks_cloned.lock().unwrap();
                let keys = tasks.range(..time).map(|x| *x.0).collect::<Vec<_>>();
                for k in keys.iter() {
                    expired.push(tasks.remove(k).unwrap());
                }
            }

            for task in expired {
                thread::spawn(move || {
                    (task.runnable)();
                });
            }

            let remaining = time.add(tick_speed).duration_since(Instant::now());

            thread::sleep(remaining);
        });

        ScheduledExecutor { tasks }
    }

    pub fn schedule_once<F>(&mut self, f: F, delay: Duration)
    where
        F: Fn() -> (),
        F: Send + Sync + 'static,
    {
        let expiry = Instant::now().add(delay);
        let task = Task {
            runnable: Box::new(f),
            expiry,
        };

        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(expiry, task);
    }

    pub fn schedule_fixed<F>(&mut self, f: F, interval: Duration)
    where
        F: Fn() -> (),
        F: Send + 'static,
    {
    }
}

struct Task {
    runnable: Box<dyn Fn() -> () + Send + Sync + 'static>,
    expiry: Instant,
}

// fn main() {
//     println!("Hello, world!");

//     let mut executor = ScheduledExecutor::new(Duration::from_millis(1000));
//     executor.schedule_once(|| println!("fluke!"), Duration::from_millis(2000));

//     thread::sleep(Duration::from_millis(50000));
// }

fn main() {
    let mut trie = HashTrie::new();
    trie.insert("foobar", 3);
    trie.insert("foobar", 5);
    dbg!(trie);
}
