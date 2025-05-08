use crate::error::{CardError, ValidationError};
use crate::utils::CardType;

#[derive(Debug, Clone)]
pub struct Card {
    number: String,
    card_type: Option<CardType>,
}

impl Card {
    pub fn new(number: &str) -> Self {
        Self {
            number: number.to_string(),
            card_type: None,
        }
    }

    pub fn number(&self) -> &str {
        &self.number
    }

    pub fn card_type(&self) -> Option<&CardType> {
        self.card_type.as_ref()
    }
}

pub trait Validatable {
    fn validate(&self) -> Result<(), CardError>;
    fn is_valid(&self) -> bool;
}

pub trait LuhnValidatable {
    fn check_luhn(&self) -> Result<bool, CardError>;
}

impl Validatable for Card {
    fn validate(&self) -> Result<(), CardError> {
        if self.number.is_empty() {
            return Err(ValidationError::EmptyInput.into());
        }

        // Check for non-numeric characters
        if let Some((pos, _)) = self.number.chars().enumerate().find(|(_, c)| !c.is_ascii_digit()) {
            return Err(ValidationError::NonNumericCharacters { position: pos }.into());
        }

        let len = self.number.len();
        let card_type = self.identify_card_type()?;

        // Validate length based on card type
        match card_type {
            CardType::Visa if !(13..=19).contains(&len) => {
                return Err(ValidationError::InvalidLength { expected: Some(16), got: len }.into());
            }
            CardType::Mastercard if len != 16 => {
                return Err(ValidationError::InvalidLength { expected: Some(16), got: len }.into());
            }
            CardType::AmericanExpress if len != 15 => {
                return Err(ValidationError::InvalidLength { expected: Some(15), got: len }.into());
            }
            CardType::Discover if !(16..=19).contains(&len) => {
                return Err(ValidationError::InvalidLength { expected: Some(16), got: len }.into());
            }
            _ => (),
        }

        Ok(())
    }

    fn is_valid(&self) -> bool {
        self.validate().is_ok() && self.check_luhn().unwrap_or(false)
    }
}

impl LuhnValidatable for Card {
    fn check_luhn(&self) -> Result<bool, CardError> {
        let mut sum = 0;
        let mut double = false;

        // Iterate through digits from right to left
        for c in self.number.chars().rev() {
            let digit = c.to_digit(10)
                .ok_or_else(|| CardError::Parse(format!("Invalid digit: {}", c)))?;

            if double {
                let doubled = digit * 2;
                sum += if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                };
            } else {
                sum += digit;
            }

            double = !double;
        }

        Ok(sum % 10 == 0)
    }
}

impl Card {
    fn identify_card_type(&self) -> Result<CardType, CardError> {
        if self.number.is_empty() {
            return Err(ValidationError::EmptyInput.into());
        }

        let first_digit = self.number.chars().next().unwrap_or('0');
        let first_two_digits = self.number.chars()
            .take(2)
            .collect::<String>();

        let card_type = match (first_digit, first_two_digits.parse::<u32>().unwrap_or(0)) {
            ('4', _) => CardType::Visa,
            ('5', 51..=55) => CardType::Mastercard,
            ('3', 34 | 37) => CardType::AmericanExpress,
            ('6', 6011) | ('6', 644..=649) | ('6', 65) => CardType::Discover,
            _ => return Err(ValidationError::InvalidCardType { prefix: first_two_digits }.into()),
        };

        Ok(card_type)
    }
} 