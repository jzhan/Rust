use std::io::Write;

fn main() {
    let mut buffer: String = String::new();

    print!("Fibonacci of: ");
    std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut buffer).
        expect("Input failed");

    match buffer.trim().parse::<u64>() {
        Ok(n) => {
            println!("Fibonacci of {} is {}", n, fib_of(&n));
        },

        Err(_e) => {
            eprintln!("{}", _e);
        }
    }
}

fn fib_of(num: &u64) -> u128 {
    if *num <= 2 {
        1
    } else {
        let fib_1 = num - 1;
        let fib_2 = num - 2;

        fib_of(&fib_1) + fib_of(&fib_2)
    }
}
