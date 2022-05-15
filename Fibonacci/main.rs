use std::io::Write;

fn main() {
    let mut buffer = String::new();

    println!("Fibonacci");
    print!("Masukkan banyak angka Fibonacci yang ingin ditampilkan: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buffer).expect("input failed");

    match buffer.trim().parse::<i64>() {
        Ok(n) => {
            let mut fib_1: i64 = 0;
            let mut fib_2: i64 = 1;

            for _i in 0 .. n {
                print!("{} ", fib_2);
                std::io::stdout().flush().unwrap();
                
                let temp = fib_2;
                fib_2 = fib_2 + fib_1;
                fib_1 = temp;
            }
        },

        Err(_e) => {
            eprintln!("{}", _e);
        }
    }
}
