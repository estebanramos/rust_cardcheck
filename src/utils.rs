use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use crate::error::{CardError, ValidationError};

#[derive(Debug, PartialEq, Clone)]
pub enum CardType {
    Visa,
    Mastercard,
    AmericanExpress,
    Discover,
    Unknown,
}

pub fn sanitize_input(card: &str) -> String {
    card.replace(['-', ' '], "").replace('\n', "")
}

pub fn identify_card_type(card: &str) -> Result<CardType, ValidationError> {
    if card.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    let first_digit = card.chars().next().unwrap_or('0');
    let first_two_digits = card.chars()
        .take(2)
        .collect::<String>();

    let card_type = match (first_digit, first_two_digits.parse::<u32>().unwrap_or(0)) {
        ('4', _) => CardType::Visa,
        ('5', 51..=55) => CardType::Mastercard,
        ('3', 34 | 37) => CardType::AmericanExpress,
        ('6', 6011) | ('6', 644..=649) | ('6', 65) => CardType::Discover,
        _ => return Err(ValidationError::InvalidCardType { prefix: first_two_digits }),
    };

    Ok(card_type)
}

pub fn validate_input(card: &str) -> Result<CardType, ValidationError> {
    if card.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    // Check for non-numeric characters
    if let Some((pos, _)) = card.chars().enumerate().find(|(_, c)| !c.is_ascii_digit()) {
        return Err(ValidationError::NonNumericCharacters { position: pos });
    }

    let len = card.len();
    let card_type = identify_card_type(card)?;

    // Validate length based on card type
    match card_type {
        CardType::Visa if !(13..=19).contains(&len) => {
            return Err(ValidationError::InvalidLength { expected: Some(16), got: len });
        }
        CardType::Mastercard if len != 16 => {
            return Err(ValidationError::InvalidLength { expected: Some(16), got: len });
        }
        CardType::AmericanExpress if len != 15 => {
            return Err(ValidationError::InvalidLength { expected: Some(15), got: len });
        }
        CardType::Discover if !(16..=19).contains(&len) => {
            return Err(ValidationError::InvalidLength { expected: Some(16), got: len });
        }
        _ => (),
    }

    Ok(card_type)
}

pub fn sum_digits(num: u32) -> Result<u32, CardError> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).ok_or_else(|| {
            CardError::Parse(format!("Failed to parse digit: {}", c))
        }))
        .sum()
}

pub fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, CardError>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}