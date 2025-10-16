use std::io;

fn get_sequence_length() -> usize {
    let mut user_input = String::new();
    let fib_sequence_length: usize;

    loop {
        user_input.clear();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse::<usize>() {
            Ok(num) => {
                fib_sequence_length = num;
                break;
            }
            Err(_) => {
                println!("Please enter a positive integer!");
                continue;
            }
        };
    }
    fib_sequence_length
}

fn fibonacci_number(position: usize, cache: &mut [usize]) -> usize {
    if position <= 1 {
        return position;
    }

    if cache[position] != 0 {
        return cache[position];
    }

    let value = fibonacci_number(position - 1, cache) + fibonacci_number(position - 2, cache);
    cache[position] = value;

    value
}

fn main() {
    println!("Hello, 🦀!\nEnter the length of Fibonacci sequence");

    let fib_sequence_length = get_sequence_length();
    if fib_sequence_length == 0 {
        println!("Sequence length is 0. Exiting.");
        return;
    }

    println!(
        "Generating Fibonacci sequence for first {} numbers",
        fib_sequence_length
    );

    let mut cache = vec![0; fib_sequence_length];

    for position in 0..fib_sequence_length {
        let fib_value = fibonacci_number(position, &mut cache);
        println!("Fibonacci number #{}: {}", position + 1, fib_value);
    }

    println!("End of the sequence . 👋");
}
