use std::io;
use std::str::FromStr;

fn main() -> io::Result<()>
{
    println!("Please input a number. I will write out a sum of all numbers from 1 to n.");
    
    let mut number_string = String::new();
    io::stdin().read_line(&mut number_string)?;

    let number = match u32::from_str(&number_string.trim())
    {
        Ok(number) => number,
        Err(_) => {
            println!("Please input only integer numbers!"); 
            std::process::exit(1);
        }
    };

    let mut sum : u32 = 0;
    for n in 1..number+1
    {
        sum += n;
    }

    println!("Final sum of all numbers from 1 to {} (including) is {}", number, sum);

    Ok(())
}
