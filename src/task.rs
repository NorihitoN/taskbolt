use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug)]
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
}
