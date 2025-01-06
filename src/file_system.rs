use crate::tasks::Task;
use std::fs;
use std::error::Error;


pub fn save_to_file(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let serialized = serde_json::to_string(&tasks)?;
    fs::write("tasks.json", serialized)?;
    Ok(())
}

pub fn load_from_file() -> Result<Vec<Task>, Box<dyn Error>> {
    let serialized = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    match serde_json::from_str(&serialized) {
        Ok(tasks) => Ok(tasks),
        Err(_) => Ok(Vec::new())
    }
}