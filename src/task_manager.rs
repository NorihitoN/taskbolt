use crate::task::{Task, TaskStatus};

#[derive(Debug, Clone)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    pub fn submit_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn cancel_task(&mut self, task_id: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = TaskStatus::Cancelled;
        }
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_task_cancel() {
        let mut task_manager = TaskManager::new();
        let task = Task::new("task2".to_string(), "Test task cancellation".to_string());

        task_manager.submit_task(task.clone());
        task_manager.cancel_task("task2");
        let tasks = task_manager.get_tasks();

        assert_eq!(tasks[0].status, TaskStatus::Cancelled);
    }
}
