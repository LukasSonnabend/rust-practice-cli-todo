mod tasks;
mod file_system;
use std::ops::ControlFlow;

use tasks::Task;
use file_system::{load_from_file, save_to_file};
use clap::{arg, Arg, Command};
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
        .subcommand(
            Command::new("remove")
            .about("Remove a todo")
            .arg(
                Arg::new("index")
                    .required(false)
                    .index(1),
            )
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
        Some(("remove", remove_matches)) => {
            if let ControlFlow::Break(_) = fun_name(&mut todos, remove_matches) {
                save_to_file(&todos).expect("Failed to save todos");
                return;
            }
        },
        _ => {
            println!("No subcommand provided");
        }
    };
    save_to_file(&todos).expect("Failed to save todos");

}

fn fun_name(todos: &mut Vec<Task>, remove_matches: &clap::ArgMatches) -> ControlFlow<()> {
    let index = remove_matches.get_one::<String>("index");
    match index {
        Some(index_string) => {
            // Pattern match e.g. unpack the result of the parse method
            if let Ok(index_parsed) = index_string.parse::<usize>() {
                if index_parsed >= todos.len() {
                    println!("Index out of bounds");
                    return ControlFlow::Break(());
                }
                todos.remove(index_parsed);
            }
        },
        None => {
            println!("Enter index of todo to remove:");
            for (i, todo) in todos.iter().enumerate() {
                println!("{}: {}", i, todo.name);
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<usize>() {
                Ok(index) => {
                    if index >= todos.len() {
                        println!("Index out of bounds");
                        return ControlFlow::Break(());
                    }
                    todos.remove(index);
                },
                Err(_) => {
                    println!("Invalid index");
                    return ControlFlow::Break(());
                }
            }
        }
    
    }
    
    ControlFlow::Continue(())
}
