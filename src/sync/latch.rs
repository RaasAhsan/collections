use std::sync::{Arc, Condvar, Mutex};

/// A blocking countdown latch.
#[derive(Debug, Clone)]
pub struct Latch {
    state: Arc<(Mutex<usize>, Condvar)>,
}

impl Latch {
    pub fn new(count: usize) -> Self {
        Latch {
            state: Arc::new((Mutex::new(count), Condvar::new())),
        }
    }

    pub fn remaining(&self) -> usize {
        let (lock, _) = &*self.state;
        *lock.lock().unwrap()
    }

    pub fn count_down(&self) {
        let (lock, cvar) = &*self.state;
        let mut count = lock.lock().unwrap();
        if *count > 0 {
            *count -= 1;
            if *count == 0 {
                cvar.notify_all();
            }
        }
    }

    pub fn wait(&self) {
        let (lock, cvar) = &*self.state;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

#[cfg(test)]
mod test {}
