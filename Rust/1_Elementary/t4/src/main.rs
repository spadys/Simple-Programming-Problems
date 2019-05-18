use std::io;
use std::str::FromStr;

fn get_sum_of_all_numbers_from_one_to_n(n: u32) -> u32
{
    let mut sum : u32 = 0;

    for i in 1..n+1
    {
        sum += i;
    }

    sum
}

#[test]
fn test()
{
    assert_eq!(get_sum_of_all_numbers_from_one_to_n(1), 1);
    assert_eq!(get_sum_of_all_numbers_from_one_to_n(2), 3);
    assert_eq!(get_sum_of_all_numbers_from_one_to_n(10), 55);
    assert_eq!(get_sum_of_all_numbers_from_one_to_n(20), 210);
}

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

    let sum = get_sum_of_all_numbers_from_one_to_n(number);
    println!("Final sum of all numbers from 1 to {} (including) is {}", number, sum);

    Ok(())
}
