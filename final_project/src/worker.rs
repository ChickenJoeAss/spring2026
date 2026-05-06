use crate::task::{Task, TaskType};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_workers(
    worker_count: usize,
    queue: Arc<Mutex<Vec<Task>>>,
    completed_tasks: Arc<Mutex<u32>>,
    io_completed: Arc<Mutex<u32>>,
    cpu_completed: Arc<Mutex<u32>>,
) {
    for id in 0..worker_count {
        let queue_clone = Arc::clone(&queue);
        let completed_clone = Arc::clone(&completed_tasks);
        let io_clone = Arc::clone(&io_completed);
        let cpu_clone = Arc::clone(&cpu_completed);

        thread::spawn(move || {
            loop {
                let task_option;

                {
                    let mut tasks = queue_clone.lock().unwrap();

                    if tasks.len() > 0 {
                        task_option = Some(tasks.remove(0));
                    } else {
                        task_option = None;
                    }
                }

                match task_option {
                    Some(task) => {
                        match task.task_type {
                            TaskType::IO => {
                                println!(
                                    "Worker {} processing IO task {}",
                                    id,
                                    task.id
                                );

                                thread::sleep(Duration::from_millis(task.duration));

                                *completed_clone.lock().unwrap() += 1;
                                *io_clone.lock().unwrap() += 1;
                            }

                            TaskType::CPU => {
                                println!(
                                    "Worker {} processing CPU task {}",
                                    id,
                                    task.id
                                );

                                thread::sleep(Duration::from_millis(task.duration));

                                *completed_clone.lock().unwrap() += 1;
                                *cpu_clone.lock().unwrap() += 1;
                            }
                        }
                    }

                    None => {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }
        });
    }
}