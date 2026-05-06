# Rust Concurrent Task Queue

## Overview
This project simulates a concurrent task queue system using Rust threads, mutexes, and shared memory.

The program includes:
- A dispatcher thread that generates tasks
- Multiple worker threads that process tasks
- A monitor thread that tracks queue size

The goal of the project is to demonstrate concurrency and synchronization in Rust.

---

## Features
- Dispatcher thread adds tasks
- Worker threads process CPU and IO tasks
- Shared task queue using Arc<Mutex<>>
- Monitor thread tracks queue size
- Counts completed tasks

---

## Output Example
Tasks completed: 100
CPU tasks: 50
IO tasks: 50

---

## Files
- `main.rs` → starts the system
- `dispatcher.rs` → generates tasks
- `worker.rs` → processes tasks
- `monitor.rs` → displays queue size
- `task.rs` → defines task structure

---

## How to Run

```bash
cargo run
```

---

## Example Output

```text
Dispatcher added task 1
Worker 0 processing IO task 1
Monitor -> Tasks in queue: 4
```

---

## Concepts Used
- Rust ownership
- Mutex synchronization
- Arc shared ownership
- Thread concurrency
- Producer-consumer model