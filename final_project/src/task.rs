
#[derive(Clone, Debug)]
pub enum TaskType {
    IO,
    CPU,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: usize,
    pub task_type: TaskType,
    pub duration: u64,
}

impl Task {
    pub fn new(id: usize, task_type: TaskType, duration: u64) -> Task {
        Task {
            id: id,
            task_type: task_type,
            duration: duration,
        }
    }
}