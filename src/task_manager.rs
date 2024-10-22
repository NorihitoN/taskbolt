#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{Task, TaskStatus};

    #[test]
    fn test_task_submission() {
        let mut task_manager = TaskManager::new();
        let task = Task::new("task1".to_string(), "Test task submission".to_string());

        task_manager.submit_task(task.clone());
        let tasks = task_manager.get_tasks();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, "task1");
        assert_eq!(tasks[0].status, TaskStatus::Pending);
    }

    fn test_task_cancel() {
        let mut task_manager = TaskManager::new();
        let task = Task::new("task2".to_string(), "Test task cancellation".to_string());

        task_manager.submit_task(task.clone());
        task_manager.cancel_task("task2".to_string());
        let tasks = task_manager.get_tasks();

        assert_eq!(tasks[0].status, TaskStatus::Cancelled);
    }
}
