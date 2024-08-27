use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn sanitize_input(card: &str) -> String {
    card.replace('-',"").replace("\n","")
}
pub fn validate_input(card: &str) -> bool {
    let len = card.len();
    if len <= 16 && !card.chars().all(char::is_numeric) {
        return false;
    }
    true
}
pub fn sum_digits(num: u32) -> u32 {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap()) // Convierte cada carácter en un dígito
        .sum() // Suma todos los dígitos
}
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}