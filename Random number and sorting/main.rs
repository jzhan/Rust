/*
    [dependencies]
    rand = "0.8.5"
*/

use std::io::Write;
use rand::Rng;

fn main() {
    let mut data: [i32; 10] = [0; 10];

    for i in 0 .. 10 {
        data[i] = rand::thread_rng().gen_range(1 .. 100);
    }

    print!("Before sorting: ");

    for i in 0 .. 10 {
        print!("{} ", data[i]);
    }
    
    data.sort();

    print!("\nAfter sorting : ");

    for i in 0 .. 10 {
        print!("{} ", data[i]);
    }

    std::io::stdout().flush().unwrap();
}
