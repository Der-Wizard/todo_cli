mod todo;
mod cli;

use todo::{Status, Todo};

fn main() {
    let mut todos: Vec<Todo> = todo::load_todos();

    loop {
        println!("//////////////////////////////");
        println!("Choose an option:");
        println!("0. Exit");
        println!("1. List todos");
        println!("2. Add todo");
        println!("3. Mark todo as In Progress");
        println!("4. Mark todo as Done");
        println!("5. Remove todo");

        println!("40. Reload todos from file");
        println!("41. Save todos to file");
        println!("//////////////////////////////");

        let choice = match cli::read_input() {
            Some(input) => input,
            None => {
                println!("Invalid input. Please try again.");
                continue;
            },
        };

        match choice.as_str() {
            "0" => {
                break;
            },
            "1" => todo::list_todos(&todos),
            "2" => {
                println!("Enter a title for your new todo:");
                let title = match cli::read_input() {
                    Some(input) => input,
                    None => {
                        println!("Invalid input. Please try again.");
                        continue;
                    }
                };
                todo::add_todo(&mut todos, &title);
            },
            "3" => {
                println!("Enter the id to mark as In Progress:");
                let id_input = match cli::read_index() {
                    Some(index) => index,
                    None => {
                        println!("Invalid input. Please try again.");
                        continue;
                    }
                };
                todo::mark_todo_status(&mut todos, id_input, Status::InProgress);
            },
            "4" => {
                println!("Enter the id to mark as Done:");
                let id_input = match cli::read_index() {
                    Some(index) => index,
                    None => {
                        println!("Invalid input. Please try again.");
                        continue;
                    }
                };
                todo::mark_todo_status(&mut todos, id_input, Status::Done);
            },
            "5" => {
                println!("Enter the id to remove:");
                let id_input = match cli::read_index() {
                    Some(index) => index,
                    None => {
                        println!("Invalid input. Please try again.");
                        continue;
                    }
                };
                todo::remove_todo(&mut todos, id_input);
            },
            "40" => {
                todos = todo::load_todos();
            },
            "41" => {
                todo::save_todos(&todos);
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }

    println!("Exiting the todo application. Goodbye!");
}
