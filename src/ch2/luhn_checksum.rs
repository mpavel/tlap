/*
Because we always want to have a plan, we should make a list of these issues and tackle them one by one:
- Knowing which digits to double
- Treating doubled numbers 10 and greater according to their individual digits
- Knowing we’ve reached the end of the number
- Reading each digit separately
*/

pub fn run() {
    println!("Luhn Checksum");
}

pub fn double_digit_value(digit: i32) -> i32 {
    let doubled = digit * 2;
    if doubled < 10 {
        return doubled;
    }
    return 1 + (doubled % 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_digit_less_than_ten() {
        assert_eq!(double_digit_value(4), 8);
    }

    #[test]
    fn double_digit_more_than_ten() {
        // 7 * 2 = 14 => 1 + 4 = 5
        assert_eq!(double_digit_value(7), 5)
    }
}
