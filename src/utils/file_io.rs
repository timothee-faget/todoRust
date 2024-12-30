use crate::utils::task;
use json;
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn get_home_path() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE").ok().map(PathBuf::from)
    } else {
        env::var("HOME").ok().map(PathBuf::from)
    }
}

pub fn check_if_task_file_exists(file_path: &PathBuf) -> bool {
    Path::new(file_path).exists()
}

pub fn create_task_file(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    fs::write(file_path, "[]")?;
    Ok(())
}

pub fn read_task_file() -> Result<Vec<task::Task>, Box<dyn Error>> {
    let mut file_path = get_home_path().unwrap();
    file_path.push("tasks.json");

    if !check_if_task_file_exists(&file_path) {
        create_task_file(&file_path)?;
    }

    let file_content = fs::read_to_string(file_path)?;
    let file_data = json::parse(&file_content)?;

    let mut tasks = vec![];
    for task in file_data.members() {
        tasks.push(task::Task::from_json(task.clone())?);
    }

    Ok(tasks)
}

pub fn write_task_file(tasks: &Vec<task::Task>) -> Result<(), Box<dyn Error>> {
    let new_tasks = tasks.to_vec();
    let contents = json::stringify(new_tasks);

    let mut file_path = get_home_path().unwrap();
    file_path.push("tasks.json");

    fs::write(file_path, contents)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::task::Task;

    #[test]
    fn home_path() {
        let home_path = PathBuf::from("/home/timothee/");

        assert_eq!(home_path, get_home_path().unwrap())
    }

    #[test]
    fn file_exists() {
        let file_path = PathBuf::from("/home/timothee/code/rust/todorust/tasks.json");

        assert_eq!(true, check_if_task_file_exists(&file_path))
    }

    #[test]
    fn create_file() {
        let file_path = PathBuf::from("/home/timothee/code/rust/todorust/new_tasks.json");

        assert!(create_task_file(&file_path).is_ok())
    }

    #[test]
    fn read_file() {
        assert!(read_task_file().is_ok())
    }

    #[test]
    fn write_file() {
        let tasks = vec![Task::new("Salut"), Task::new("Hello"), Task::new("Hola")];
        assert!(write_task_file(&tasks).is_ok())
    }
}
