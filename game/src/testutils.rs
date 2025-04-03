use std::str::Chars;

pub struct NumbersParser<'a> {
    input: Chars<'a>,
}
impl<'a> NumbersParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
        }
    }

    pub fn next_number(&mut self) -> Option<u32> {
        let mut number = None;
        while let Some(digit) = self.input.next() {
            if digit.is_ascii_digit() {
                number = Some(number.unwrap_or(0) * 10 + (digit as u32 - '0' as u32));
            } else if digit.is_whitespace() {
                if number.is_some() {
                    return number;
                } else {
                    // seems we have multiple whitespaces in a row, haven't seen a number yet. Just skip the whitespace
                }
            } else {
                panic!("Unexpected character in input: '{}'", digit);
            }
        }
        // End of string
        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut parser = NumbersParser::new("");
        assert_eq!(parser.next_number(), None); // End of input
        assert_eq!(parser.next_number(), None); // End of input
    }

    #[test]
    fn test_numbers_parser() {
        let mut parser = NumbersParser::new("123 456 789");
        assert_eq!(parser.next_number(), Some(123));
        assert_eq!(parser.next_number(), Some(456));
        assert_eq!(parser.next_number(), Some(789));
        assert_eq!(parser.next_number(), None); // End of input
        assert_eq!(parser.next_number(), None); // End of input
    }

    #[test]
    fn test_whitespace() {
        let mut parser = NumbersParser::new("  42   7  89 ");
        assert_eq!(parser.next_number(), Some(42));
        assert_eq!(parser.next_number(), Some(7));
        assert_eq!(parser.next_number(), Some(89));
        assert_eq!(parser.next_number(), None); // End of input
    }
}
