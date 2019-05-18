use std::io;
use std::str::FromStr;

fn get_sum_of_all_multiples_from_one_to_n(n: u32, divisors: &[u32]) -> u32
{
    let mut sum : u32 = 0;

    for i in 1..n+1
    {
        let mut is_multiple = false;
        for div in divisors
        {
            is_multiple |= i % div == 0;

            if is_multiple
            {
                sum += i;
                break;
            }
        }
    }

    sum
}

#[test]
fn test_for_all_numbers()
{
    assert_eq!(get_sum_of_all_multiples_from_one_to_n(1,  &[1   ]), 1);
    assert_eq!(get_sum_of_all_multiples_from_one_to_n(2,  &[1   ]), 3);
    assert_eq!(get_sum_of_all_multiples_from_one_to_n(10, &[1   ]), 55);
    assert_eq!(get_sum_of_all_multiples_from_one_to_n(20, &[1   ]), 210);
}

#[test]
fn test_for_3_and_5()
{
    assert_eq!(get_sum_of_all_multiples_from_one_to_n(1,  &[3, 5]), 0);
    assert_eq!(get_sum_of_all_multiples_from_one_to_n(3,  &[3, 5]), 3);
    assert_eq!(get_sum_of_all_multiples_from_one_to_n(10, &[3, 5]), 33);
    assert_eq!(get_sum_of_all_multiples_from_one_to_n(17, &[3, 5]), 60);
}

fn main() -> io::Result<()>
{
    println!("Please input a number. I will write out a sum of all multiples of three or five from 1 to n.");
    
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

    let sum = get_sum_of_all_multiples_from_one_to_n(number, &[3, 5]);

    println!("Final sum of all multiples of three or five from 1 to {} (including) is {}", number, sum);

    Ok(())
}
