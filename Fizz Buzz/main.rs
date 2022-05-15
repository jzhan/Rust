use std::io::Write;

fn main() {
    let mut buffer: String = String::new();

    for i in 1 .. 37 {
        print!("{}, ", if i % 3 == 0 && i % 5 == 0 {
                            "Fizz Buzz".to_string()
                        } else if i % 3 == 0 {
                            "Fizz".to_string()
                        } else if i % 5 == 0 {
                            "Buzz".to_string()
                        } else {
                            i.to_string().clone()
                        });
        std::io::stdout().flush().unwrap();

        buffer.clear();
    }
}
