use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Wtf? How did IO operation even fail???");
    name = name.trim().to_string();

    println!("Hello {}. Nice to meet you.", name);
    // TODO learn how to use io::stdout()
}
