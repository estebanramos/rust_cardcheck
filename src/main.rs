pub mod utils;
mod validation;
use clap::{Parser, ArgGroup};


#[derive(Parser)]
#[command(name = "Mi Programa")]
#[command(about = "Un ejemplo con grupos de argumentos en clap", long_about = None)]
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


fn main() {
    let cli = Args::parse();
    if let Some(mut card_number) = cli.card {
            card_number = utils::sanitize_input(&card_number).to_string();
            if utils::validate_input(&card_number) {
                println!("Card provided: {:?}", &card_number);
                if validation::check_luhn(&card_number) {
                    println!("[#] {:?} - CARD VALID", card_number);
                }
                else {
                    println!("[#] {:?} - CARD NOT VALID", card_number);
                }
            } else {
                eprintln!("Bad Format for Card Number");
            }       
    } 
    else if let Some(file_path) = cli.file {
        if let Ok(lines) = utils::read_lines(file_path) {
            for line in lines.flatten() {
                let card_number = utils::sanitize_input(&line).to_string().replace('"',"");
                if validation::check_luhn(&card_number) {
                    println!("[#] {:?} - CORRECT",line);
                }
            }
        }
    
    }
}

