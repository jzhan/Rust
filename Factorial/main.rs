use std::io::Write;

fn main() {
    let mut buffer: String = String::new();

    print!("Factorial of: ");
    std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut buffer).
        expect("Input failed");

    match buffer.trim().parse::<u64>() {
        Ok(mut n) => {
            let mut factorial: u128 = 1;
            n = n + 1;

            for i in 1 .. n {
                factorial = factorial * i as u128;
            }

            print!("Result: {}", factorial);
        },

        Err(_e) => {
            eprintln!("{}", _e);
        }
    }
}
