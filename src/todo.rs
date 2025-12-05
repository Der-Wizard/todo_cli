use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub status: Status
}

pub fn list_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No todos found.");
        return;
    }

    for (i, todo) in todos.iter().enumerate() {
        let status = match todo.status {
            Status::Todo => "[ ]",
            Status::InProgress => "[-]",
            Status::Done => "[X]",
        };
        println!("{} {}: {}", i + 1, status, todo.title);
    }
}

pub fn add_todo(todos: &mut Vec<Todo>, title: &str) {
    let todo = Todo {
        title: String::from(title),
        status: Status::Todo,
    };
    todos.push(todo);
}

pub fn mark_todo_status(todos: &mut Vec<Todo>, index: usize, status: Status) {
    if let Some(todo) = todos.get_mut(index) {
        todo.status = status;
        println!("Updated todo: {} to status: {:?}", todo.title, todo.status);
    }
}

pub fn remove_todo(todos: &mut Vec<Todo>, index: usize) {
    if let Some(todo) = todos.get(index) {
        println!("Removing todo: {}", todo.title);
        todos.remove(index);
    }
    else {
        println!("No todo found at the given index.");
    }
}

const DB_PATH: &str = "files/todos.json";

pub fn save_todos(todos: &Vec<Todo>) {
    let todos = match serde_json::to_string_pretty(&todos) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to serialize todos: {}", e);
            return;
        }
    };

    match fs::write(DB_PATH, todos) {
        Ok(_) => println!("Todos saved successfully."),
        Err(e) => println!("Failed to save todos: {}", e),
    };
}

pub fn load_todos() -> Vec<Todo> {
    let data = match fs::read_to_string(DB_PATH) {
        Ok(content) => content,
        Err(_) => {
            println!("No existing todo database found. Starting fresh.");
            return Vec::new();
        }
    };

    match serde_json::from_str::<Vec<Todo>>(&data) {
        Ok(todos) => todos,
        Err(e) => {
            println!("Failed to parse todos: {}", e);
            Vec::new()
        }
    }
}