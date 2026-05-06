use crate::task::Task;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_monitor(
    queue: Arc<Mutex<Vec<Task>>>,
) {
    thread::spawn(move || {
        loop {

            {
                let tasks = queue.lock().unwrap();

                println!(
                    "Monitor -> Tasks in queue: {}",
                    tasks.len()
                );
            }

            thread::sleep(Duration::from_millis(500));
        }
    });
}