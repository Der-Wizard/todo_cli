pub fn read_input() -> Option<String> {
    let mut choice = String::new();
    let result;
    match std::io::stdin().read_line(&mut choice) {
        Ok(_) => result = Some(choice.trim().to_string()),
        Err(_) => {
            println!("Failed to read input. Please try again.");
            result = None;
        }
    }

    result
}

pub fn read_index() -> Option<usize> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<usize>() {
            Ok(index) => Some(index - 1), // Convert to zero-based index
            Err(_) => {
                println!("Please enter a valid number.");
                None
            }
        },
        Err(_) => {
            println!("Failed to read input. Please try again.");
            None
        }
    }
}