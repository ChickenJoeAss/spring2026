use crate::task::{Task, TaskType};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_dispatcher(
    queue: Arc<Mutex<Vec<Task>>>,
) {
    thread::spawn(move || {
        for i in 0..100 {

            let task_type;

            if i % 2 == 0 {
                task_type = TaskType::IO;
            } else {
                task_type = TaskType::CPU;
            }

            let task = Task::new(
                i,
                task_type,
                200,
            );

            {
                let mut tasks = queue.lock().unwrap();
                tasks.push(task);
            }

            println!("Dispatcher added task {}", i);

            thread::sleep(Duration::from_millis(50));
        }
    });
}