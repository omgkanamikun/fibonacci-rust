fn main() {
    println!("Will print Fibonacci sequence for first {} numbers", NUMBER);

    let mut cache: [usize; NUMBER] = [0; NUMBER];

    const NUMBER: usize = 51;

    for i in 0..NUMBER {
        println!("{}", fibonacci_number(i, &mut cache));
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