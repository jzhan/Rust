// use std::io;
use std::io::Write; 

fn main() {
    let val: i32;
    
    loop {
        print!("Enter a number: ");
        /* stdout is frequently line-buffered by default 
           so it may be necessary to use io::stdout().flush()
        */ 
        std::io::stdout().flush().unwrap();
        
        let mut buffer: String = String::new();

        // io::stdin().read_line(&mut buffer).expect("input failed");

        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                match buffer.trim().parse::<i32>() {
                    Ok(buffer) => {
                        val = buffer;

                        break;
                    },

                    Err(_) => 
                        eprintln!("Invalid input. Input must be a number.\n"),
                };
            },

            Err(_) => eprintln!("Input failed"),
        }       
    }
    
    println!("\nNumber you entered is {}.", val);
}
