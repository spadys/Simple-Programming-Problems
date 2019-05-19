fn print_multiplication_row(row: u32, max: u32)
{
    print!("{0: <4}", row);

    for i in 1..max+1
    {
        let multiplication = row * i;
        print!(" {0: <3}", multiplication);
    }

    println!("");
}

fn print_multiplication_table(bounds: (u32, u32))
{
    print!("x   ");

    for i in bounds.0..bounds.1+1
    {
        print!(" {0: <3}", i);
    }

    println!("\n");

    for i in bounds.0..bounds.1+1
    {
        print_multiplication_row(i, bounds.1);
    }
}

fn main() {
    print_multiplication_table((1, 12));
}
