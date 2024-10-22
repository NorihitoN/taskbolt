use crate::task::{Task, TaskStatus};

pub struct Worker {
    pub id: String,
    pub is_busy: bool,
}

impl Worker {
    pub fn new(id: String) -> Self {
        Worker { id, is_busy: false }
    }

    pub fn execute_task(&mut self, task: &mut Task) {
        self.is_busy = true;
        task.status = TaskStatus::Running;

        task.status = TaskStatus::Completed;
        self.is_busy = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_executes_task() {

        let mut task = Task::new("task1".to_string(), "Test task execution".to_string());
        let mut worker = Worker::new("worker1".to_string());

        worker.execute_task(&mut task);

        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(worker.is_busy, false);
    }
}
