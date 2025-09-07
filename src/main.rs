mod decode_text;
mod enter_text;
mod scramble_text;

use std::io::{Write};

fn main() {
    loop {
        print!("Enter 's' to scramble text, 'd' to decode text, or anything else to quit: ");
        std::io::stdout().flush().unwrap();

        let mut instruction = String::new();

        std::io::stdin()
            .read_line(&mut instruction)
            .expect("Failed to read input");
        
        match instruction.trim() {
            "d" => {
                print!("\nEnter text to decode: ");
                std::io::stdout().flush().unwrap();
                
                let input = enter_text::enter_text();
                let matches = decode_text::decode_text(&input);
                
                if matches.is_empty() {
                    println!("\nNo matches found");
                } else {

                    println!("\nResults:");
                    
                    for word in matches {
                        println!("{}", word)
                    }
                }
                    
                println!();
            }
            "s" => {
                print!("\nEnter text to scramble: ");
                std::io::stdout().flush().unwrap();

                let input = enter_text::enter_text();
                let mut scramble = scramble_text::scramble_text(&input);
                println!("Result: {}", scramble);
                
                loop {
                    print!("\nEnter 'y' to scramble again, anything else to return to menu: ");
                    std::io::stdout().flush().unwrap();

                    let mut repeat = String::new();

                    std::io::stdin()
                        .read_line(&mut repeat)
                        .expect("Failed to read input");

                    match repeat.trim() {
                        "y" => {
                            println!("\nText to scramble: {}", input);
                            scramble = scramble_text::scramble_text(&input);
                            println!("Result: {}", scramble);
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => {
                println!("Quitting...");
                break;
            }
        }
    }
}
