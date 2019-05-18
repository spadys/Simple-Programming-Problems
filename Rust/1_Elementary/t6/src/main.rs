use std::io;
use std::str::FromStr;

fn get_sum_of_all_numbers_from_one_to_n(n: u64) -> u64
{
    let mut sum : u64 = 0;

    for i in 1..n+1
    {
        sum += i;
    }

    sum
}

fn get_product_of_all_numbers_from_one_to_n(n: u64) -> u64
{
    let mut product : u64 = 1;

    for i in 1..n+1
    {
        product *= i;
    }

    product
}

#[test]
fn test_sum()
{
    assert_eq!(get_sum_of_all_numbers_from_one_to_n(1), 1u64);
    assert_eq!(get_sum_of_all_numbers_from_one_to_n(2), 3u64);
    assert_eq!(get_sum_of_all_numbers_from_one_to_n(10), 55u64);
    assert_eq!(get_sum_of_all_numbers_from_one_to_n(20), 210u64);
}

#[test]
fn test_product()
{
    assert_eq!(get_product_of_all_numbers_from_one_to_n(1), 1u64);
    assert_eq!(get_product_of_all_numbers_from_one_to_n(2), 2u64);
    assert_eq!(get_product_of_all_numbers_from_one_to_n(3), 6u64);
    assert_eq!(get_product_of_all_numbers_from_one_to_n(5), 120u64);
    assert_eq!(get_product_of_all_numbers_from_one_to_n(10), 3_628_800u64);
}

fn do_sum(number: u64) -> ()
{
    let sum = get_sum_of_all_numbers_from_one_to_n(number);
    println!("Final sum of all numbers from 1 to {} (including) is {}", number, sum);
}

fn do_product(number: u64) -> ()
{
    let product = get_product_of_all_numbers_from_one_to_n(number);
    println!("Final product of all numbers from 1 to {} (including) is {}", number, product);
}

fn main() -> io::Result<()>
{
    println!("Please input a number. I will write out a either a sum or a product of all numbers from 1 to n.");
    
    let mut number_string = String::new();
    io::stdin().read_line(&mut number_string)?;

    let number = match u64::from_str(&number_string.trim())
    {
        Ok(number) => number,
        Err(_) => {
            println!("Please input only integer numbers!"); 
            std::process::exit(1);
        }
    };

    println!("Choose between sum and product. Type s(um) for sum and p(roduct) for product");

    let mut choice_string = String::new();
    io::stdin().read_line(&mut choice_string)?;

    match &choice_string.trim()[0..1]
    {
        "s" => do_sum(number),
        "p" => do_product(number),
        _   => {
            println!("Please input only s(um) or p(roduct)"); 
            std::process::exit(1);
        }
    }

    Ok(())
}
