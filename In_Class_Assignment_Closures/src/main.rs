use std::{thread, time::Duration};

// Task 1: Basic Closure
fn task1() {
    let operation = |a: i32, b: i32| {
        a * b
    };

    println!("Task 1 Result: {}", operation(10, 5));
}

// Task 2: Environment Capture
fn track_changes() {
    let mut tracker = 0;

    let mut update = || {
        tracker += 1;
        println!("Tracker: {}", tracker);
    };

    update();
    update();
}

// Task 3: Vector Transformation
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();

    for x in vec {
        result.push(f(x));
    }

    result
}

fn task3() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        if x > 2 {
            0
        } else {
            x
        }
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}

// Task 5: Lazy Computation (Cache)
struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                println!("Computing (this will take 2 seconds)...");
                thread::sleep(Duration::from_secs(2));

                let result = (self.computation)();
                self.value = Some(result.clone());

                result
            }
        }
    }
}

fn task5() {
    let mut cache = ComputeCache::new(|| {
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());

    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}

// Main

fn main() {
    println!("--- Task 1 ---");
    task1();

    println!("\n--- Task 2 ---");
    track_changes();

    println!("\n--- Task 3 ---");
    task3();

    println!("\n--- Task 5 ---");
    task5();
}