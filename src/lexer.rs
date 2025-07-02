#[derive(Debug, PartialEq)]
pub enum Tokens {
    Left,            // <
    Right,           // >
    Plus,            // +
    Minus,           // -
    LParen,          // [
    RParen,          // ]
    Read,            // ,
    Write,           // .
}

pub fn tokenize(code: &str) -> Vec<Tokens> {
    let mut result: Vec<Tokens> = vec![];
    
    for symbol in code.chars() {
        let token: Tokens = match symbol {
            '<' => Tokens::Left,
            '>' => Tokens::Right,
            '+' => Tokens::Plus,
            '-' => Tokens::Minus,
            '[' => Tokens::LParen,
            ']' => Tokens::RParen,
            ',' => Tokens::Read,
            '.' => Tokens::Write,
            _ => continue // comments
        };

        result.push(token);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_sequence() {
        assert_eq!(tokenize("<>+-[],."), vec![
            Tokens::Left,
            Tokens::Right,
            Tokens::Plus,
            Tokens::Minus,
            Tokens::LParen,
            Tokens::RParen,
            Tokens::Read,
            Tokens::Write
        ]);
    }

    #[test]
    fn test_tokenize_ignore_comments() {
        assert_eq!(tokenize(">>hel.,l.o++< wor++ld -"), vec![
            Tokens::Right,
            Tokens::Right,
            Tokens::Write,
            Tokens::Read,
            Tokens::Write,
            Tokens::Plus,
            Tokens::Plus,
            Tokens::Left,
            Tokens::Plus,
            Tokens::Plus,
            Tokens::Minus
        ]);
    }
}