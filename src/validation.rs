use crate::error::CardError;
use crate::utils;

pub fn check_luhn(card: &str) -> Result<bool, CardError> {
    let mut card_reversed = card.chars().rev().collect::<String>();
    let first_digit = card_reversed.chars().nth(0).unwrap().to_digit(10).unwrap();
    let card_reversed = &card_reversed[1..];
    let mut total_sum_undoubled: u32 = 0;
    let mut total_sum_doubled: u32 = 0;

    for (i, n) in card_reversed.chars().enumerate() {
        let digit = n.to_digit(10).unwrap();
        if i % 2 == 0 {
            let doubled = digit * 2;
            if doubled >= 9 {
                match utils::sum_digits(doubled) {
                    Ok(sum) => total_sum_doubled += sum,
                    Err(e) => return Err(e),
                }
            } else {
                total_sum_doubled += doubled;
            }
        } else {
            total_sum_undoubled += digit;
        }
    }

    let total = total_sum_doubled + total_sum_undoubled + first_digit;
    Ok(total % 10 == 0)
}