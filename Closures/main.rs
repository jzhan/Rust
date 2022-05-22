use std::io::Write;

fn main() {
    let mut buffer: String = String::new();

    print!("Input a number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();

    match buffer.trim().parse::<i32>() {
        Ok(num) => {
            let odd_or_even = | x: i32 | -> &str {
                if x % 2 == 0 {
                    "EVEN"
                } else {
                    "ODD"
                }
            };

            println!("{}", odd_or_even(num));
        },

        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
