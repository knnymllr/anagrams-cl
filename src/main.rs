mod decode_text;
mod enter_text;
mod scramble_text;

use std::io::{Read, Write};

fn main() {
    loop {
        print!("Enter 's' to scramble text, 'd' to decode text, or anything else to quit: ");
        std::io::stdout().flush().unwrap();

        let mut instruction = [0u8; 2];

        std::io::stdin()
            .read_exact(&mut instruction)
            .expect("Failed to read character");

        let ch = instruction[0].to_ascii_lowercase() as char;
        
        match ch {
            'd' => {
                print!("\nEnter text to decode: ");
                std::io::stdout().flush().unwrap();
                
                let input = enter_text::enter_text();
                let matches = decode_text::decode_text(&input);
                
                println!("\nResults:");

                for word in matches {
                    println!("{}", word)
                }

                println!();
            }
            's' => {
                print!("\nEnter text to scramble: ");
                std::io::stdout().flush().unwrap();

                let input = enter_text::enter_text();
                let scramble = scramble_text::scramble_text(&input);
                println!("Result: {}", scramble);
            }
            _ => {
                println!("Quitting...");
                break;
            }
        }
    }
}
