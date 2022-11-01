use std::io;
use std::num::ParseIntError;

fn main() {
    println!("Enter number for Fibonacci sequence length");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("error while reading the line");

    let res: Result<usize, ParseIntError> = number.trim().parse::<usize>();

    let number = match res {
        Ok(int) => int,
        Err(core) => {
            println!("using fallback value, error: {core}");
            56
        }
    };

    println!("Fibonacci sequence for first {} numbers", number);

    let mut cache = vec![0; number];

    for i in 0..number {
        println!("fibonacci_number: {}", fibonacci_number(i, &mut cache));
    }

    println!("The end")
}

fn fibonacci_number(number: usize, arr: &mut [usize]) -> usize {
    if number <= 1 { return number; }

    if arr[number] != 0 && arr.len() > 0 { return arr[number]; }

    let i = fibonacci_number(number - 1, arr) + fibonacci_number(number - 2, arr);

    arr[number] = i;

    i
}