use std::io;
use std::num::ParseIntError;

fn main() {
    println!("Enter number for Fibonacci sequence length");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("error while reading the line");

    let res: Result<usize, ParseIntError> = number.trim().parse::<usize>();

    let number = res.unwrap_or_else(|err_value| {
        println!("using fallback value, error: {}", err_value);
        96
    });

    println!("Fibonacci sequence for first {} numbers", number);

    let mut cache = vec![0; number];

    let mut count = 1;

    for i in 0..number {
        println!(
            "fibonacci_number {} is '{}'",
            count,
            fibonacci_number(i, &mut cache)
        );
        count += 1;
    }

    println!("The end")
}

fn fibonacci_number(number: usize, arr: &mut [usize]) -> usize {
    if number <= 1 {
        return number;
    }

    if arr[number] != 0 && !arr.is_empty() {
        return arr[number];
    }

    let i = fibonacci_number(number - 1, arr) + fibonacci_number(number - 2, arr);

    arr[number] = i;

    i
}
