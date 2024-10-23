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

    pub fn fail_task(&mut self, task: &mut Task) {
        self.is_busy = true;
        task.status = TaskStatus::Failed;
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

    #[test]
    fn test_worker_busy() {
        let mut task = Task::new("task2".to_string(), "Test worker busy".to_string());
        let mut worker = Worker::new("worker2".to_string());

        worker.execute_task(&mut task);
        assert_eq!(worker.is_busy, false);
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[test]
    fn test_worker_handles_task_failure() {
        let mut task = Task::new("task3".to_string(), "Test task failure".to_string());
        let mut worker = Worker::new("worker3".to_string());

        worker.fail_task(&mut task);
        assert_eq!(task.status, TaskStatus::Failed);
        assert_eq!(worker.is_busy, false);
    }
}
