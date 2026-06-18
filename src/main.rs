use std::io::{self, Write};

fn main() {
    print!("Введите ваше имя: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Привет, {}! Добро пожаловать в Rust на Android!", name.trim());
}
