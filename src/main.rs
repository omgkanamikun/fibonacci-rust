use std::io;

fn get_sequence_length() -> usize {
    let mut user_input = String::new();

    loop {
        user_input.clear();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse::<usize>() {
            Ok(num) => break num,
            Err(_) => eprint!("Please enter a positive integer!"),
        }
    }
}

fn fibonacci_recursive(position: usize, cache: &mut [Option<usize>]) -> usize {
    if let Some(value) = cache[position] {
        return value;
    }

    let value = match position {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(position - 1, cache)
            .checked_add(fibonacci_recursive(position - 2, cache))
            .expect("Fibonacci value exceeds usize"),
    };

    cache[position] = Some(value);

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

    let mut cache = vec![None; fib_sequence_length];

    for position in 0..fib_sequence_length {
        let fib_value = fibonacci_recursive(position, &mut cache);
        println!("Fibonacci number #{}: {fib_value}", position + 1);
    }

    println!("End of the sequence . 👋");
}

#[cfg(test)]
mod tests {
    use super::fibonacci_recursive;

    #[test]
    fn returns_base_cases() {
        let mut cache = [None; 3];

        assert_eq!(fibonacci_recursive(0, &mut cache), 0);
        assert_eq!(fibonacci_recursive(1, &mut cache), 1);
    }

    #[test]
    fn returns_expected_sequence_value_and_populates_cache() {
        let mut cache = [None; 8];

        let value = fibonacci_recursive(7, &mut cache);

        assert_eq!(value, 13);
        assert_eq!(cache[7], Some(13));
        assert_eq!(cache[6], Some(8));
    }

    #[test]
    fn provides_until_usize_limit() {
        let mut cache = [None; 94];

        assert_eq!(fibonacci_recursive(92, &mut cache), 7540113804746346429);
        assert_eq!(fibonacci_recursive(93, &mut cache), 12200160415121876738);
    }

    #[test]
    #[should_panic(expected = "Fibonacci value exceeds usize")]
    fn panics_on_overflow() {
        let mut cache: Vec<Option<usize>> = vec![None; 95];
        fibonacci_recursive(94, &mut cache);
    }
}
