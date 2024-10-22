use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub retries: u8,
}

impl Task {
    pub fn new(id: String, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Pending,
            retries: 3,
        }
    }

    pub fn retry(&mut self) {
        if self.retries > 0 {
            self.retries -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_task() {
        let task = Task::new("task1".to_string(), "Test task creation".to_string());

        assert_eq!(task.id, "task1");
        assert_eq!(task.description, "Test task creation");
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.retries, 3);
    }

    #[test]
    fn test_update_task_status() {
        let mut task = Task::new("task2".to_string(), "Test task status update".to_string());
        assert_eq!(task.status, TaskStatus::Pending);

        task.status = TaskStatus::Running;
        assert_eq!(task.status, TaskStatus::Running);

        task.status = TaskStatus::Completed;
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[test]
    fn test_no_retries_left() {
        let mut task = Task::new("task3".to_string(), "Test retries".to_string());

        task.retries = 0;
        assert_eq!(task.retries, 0);

        task.retry();
        assert_eq!(task.retries, 0);
    }
}
