use std::io;

pub fn get_string(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
   
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read string\n");

    input.trim().into()
}

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
