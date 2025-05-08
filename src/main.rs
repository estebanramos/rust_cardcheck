pub mod utils;
pub mod error;
pub mod card;
mod validation;
use clap::{Parser, ArgGroup};
use card::{Card, Validatable, LuhnValidatable};
use error::CardError;

#[derive(Parser)]
#[command(name = "Rust Card Check")]
#[command(about = "Check valid credit/debit cards using Luhn's Algorithm", long_about = None)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["card", "file"])
))]
struct Args {
    /// Card Number
    #[arg(long)]
    card: Option<String>,

    /// Path to file containing card numbers
    #[arg(long)]
    file: Option<String>,
}

fn process_card(card_number: &str) -> Result<(), CardError> {
    let card = Card::new(card_number);
    
    println!("Card provided: {}", card.number());
    
    match card.validate() {
        Ok(_) => {
            if let Ok(is_valid) = card.check_luhn() {
                if is_valid {
                    println!("[✓] Card is VALID");
                } else {
                    println!("[✗] Card is NOT VALID");
                }
            }
        }
        Err(e) => return Err(e),
    }
    
    Ok(())
}

fn main() {
    let cli = Args::parse();
    
    if let Some(card_number) = cli.card {
        let sanitized_number = utils::sanitize_input(&card_number);
        if let Err(e) = process_card(&sanitized_number) {
            eprintln!("[✗] Error: {}", e);
        }
    } 
    else if let Some(file_path) = cli.file {
        match utils::read_lines(file_path) {
            Ok(lines) => {
                for (line_num, line) in lines.enumerate() {
                    match line {
                        Ok(card_number) => {
                            let sanitized_number = utils::sanitize_input(&card_number).replace('"', "");
                            println!("\nProcessing card #{}:", line_num + 1);
                            if let Err(e) = process_card(&sanitized_number) {
                                eprintln!("[✗] Error processing card #{}: {}", line_num + 1, e);
                            }
                        }
                        Err(e) => eprintln!("[✗] Error reading line {}: {}", line_num + 1, e),
                    }
                }
            }
            Err(e) => eprintln!("[✗] Error reading file: {}", e),
        }
    }
}

