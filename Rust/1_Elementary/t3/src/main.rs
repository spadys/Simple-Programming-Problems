use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Wtf? How did IO operation even fail???");
    let trimmed_name = name.trim();

    match trimmed_name
    {
        "Alice" => println!("Hello {}. Nice to meet you.", trimmed_name),
        "Bob"   => println!("Hello {}. Nice to meet you.", trimmed_name),
        _       => println!("Warning! Access denied!"),
    }
}
