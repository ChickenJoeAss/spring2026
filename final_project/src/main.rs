mod task;
mod worker;
mod dispatcher;
mod monitor;

use task::Task;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {

    let start = Instant::now();
    let queue = Arc::new(Mutex::new(Vec::<Task>::new()));

    let completed_tasks = Arc::new(Mutex::new(0));
    let io_completed = Arc::new(Mutex::new(0));
    let cpu_completed = Arc::new(Mutex::new(0));

    worker::start_workers(
        2,
        Arc::clone(&queue),
        Arc::clone(&completed_tasks),
        Arc::clone(&io_completed),
        Arc::clone(&cpu_completed),
    );

    dispatcher::start_dispatcher(
        Arc::clone(&queue),
    );

    monitor::start_monitor(
        Arc::clone(&queue),
    );

    thread::sleep(Duration::from_secs(30));

    println!("Program finished.");
    println!("Total runtime: {:?}", start.elapsed());
    println!("Tasks completed: {}", *completed_tasks.lock().unwrap());
    println!("CPU tasks: {}", *cpu_completed.lock().unwrap());
    println!("IO tasks: {}", *io_completed.lock().unwrap());
}