use std::io;

/// returns a String data type
pub fn get_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read string\n");

    input.trim().into()
}

/// returns a usize
pub fn get_usize(prompt: &str) -> usize {
    let mut input: String;

    loop {
        input = get_string(prompt);
        break match input.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse input\n");
                continue;
            }
        };
    }
}

/// returns a bool
pub fn get_bool(prompt: &str) -> bool {
    loop {
        match get_string(prompt).to_lowercase().as_str() {
            "y" | "yes" => break true,
            "n" | "no" => break false,
            _ => {
                println!("Error: y/es or n/o");
                continue;
            }
        }
    }
}
