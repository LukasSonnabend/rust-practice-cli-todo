mod tasks;
mod file_system;
use tasks::Task;
use file_system::{load_from_file, save_to_file};
use clap::{arg, Command};
use chrono::{DateTime, Duration, Utc};

fn cli() -> Command {
    Command::new("todo-mngr")
        .version("0.1.0")
        .author("Lukas Sonnabend")
        .about("Manage your TODOs")
        .subcommand(
            Command::new("add")
                .about("Add a new TODO")
                .arg(arg!(<text> "Text of the Todo"))
                
        )
}

fn main() {
    let matches = cli().get_matches();
    let mut todos: Vec<Task> = load_from_file().expect("Failed to load todos");
    match matches.subcommand() {
        Some(("add", add_matches)) => {
            let text = add_matches.get_one::<String>("text").map(|t| t.to_string()).unwrap();
            todos.push(Task {
                id: todos.len() as i32,
                name: text.clone(),
                date: Utc::now(),
                done: false,
            });
            println!("Added new TODO: {}", text);
        },
        _ => {
            println!("No subcommand provided");
        }
    };
    save_to_file(&todos).expect("Failed to save todos");

}
