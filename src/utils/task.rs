use std::error::Error;
use std::fmt;

use json::JsonValue;

pub struct Task {
    title: String,
    completed: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = if self.completed { "x" } else { " " };
        write!(f, "[{}] {}", state, self.title)
    }
}

impl Clone for Task {
    fn clone(&self) -> Self {
        Task {
            title: self.title.clone(),
            completed: self.completed.clone(),
        }
    }
}

impl Into<JsonValue> for Task {
    fn into(self: Task) -> JsonValue {
        let mut json_value = JsonValue::new_object();
        json_value.insert("title", self.title).unwrap();
        json_value.insert("completed", self.completed).unwrap();

        json_value
    }
}

impl Task {
    pub fn new(title: &str) -> Task {
        Task {
            title: String::from(title),
            completed: false,
        }
    }

    pub fn from_json(task: JsonValue) -> Result<Task, Box<dyn Error>> {
        if task.has_key("title") && task.has_key("completed") {
            let completed = if task["completed"].to_string() == "true" {
                true
            } else {
                false
            };
            Ok(Task {
                title: task["title"].to_string(),
                completed,
            })
        } else {
            Err(Box::new(MissingFieldError {}))
        }
    }

    pub fn complete(&mut self) {
        self.completed = !self.completed;
    }
}

#[derive(Debug)]
struct MissingFieldError {}

impl fmt::Display for MissingFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Champ manquant lors de l'import d'une tâche, vérifier le fichier."
        )
    }
}

impl Error for MissingFieldError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task() {
        let task = Task::new("Salut");

        assert_eq!(task.title, "Salut");
        assert_eq!(task.completed, false)
    }

    #[test]
    fn build_task_from_json() {
        let mut json_task = JsonValue::new_object();
        json_task.insert("title", "Salut").unwrap();
        json_task.insert("completed", true).unwrap();

        let task = Task::from_json(json_task).unwrap();

        assert_eq!(task.completed, true);
        assert_eq!(task.title, "Salut");
    }

    #[test]
    fn build_task_from_json_fail() {
        let mut json_task = JsonValue::new_object();
        json_task.insert("title", "Salut").unwrap();

        let task = Task::from_json(json_task);

        assert!(task.is_err())
    }

    #[test]
    fn print_task() {
        let mut task = Task::new("Print task");
        assert_eq!(format!("{}", task), "[ ] Print task");
        task.completed = true;
        assert_eq!(format!("{}", task), "[x] Print task");
    }

    #[test]
    fn complete_task() {
        let mut task = Task::new("Print task");
        task.complete();

        assert!(task.completed)
    }
}
