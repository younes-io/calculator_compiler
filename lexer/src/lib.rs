/// The lexer module
mod lexer {
    use std::cell::{Cell, RefCell};

    enum TokenType {
        INT(usize), // an integer
        ADD,        // the addition symbol : '+'
        MIN,        // the subtraction symbol : '-'
        MULT,       // the multiplication symbol : '*'
        DIV,        // the division symbol : '-'
        WS,         // Whitespace
        UNKNOWN,    // An invalid token (e.g. '~')
        EOF,        // End Of File
    }

    pub struct Scanner<'a> {
        current_position: Cell<usize>,
        source: &'a str,
        pub tokens: RefCell<Vec<&'a str>>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(input: &'a str) -> Self {
            Scanner {
                current_position: Cell::new(0),
                source: input,
                tokens: RefCell::new(Vec::<&'a str>::new()),
            }
        }

        fn is_eof(&self) -> bool {
            self.current_position.get() >= self.source.len()
        }

        fn recognize(c: Option<char>) -> TokenType {
            match c {
                Some(ch) if ch.is_digit(10) => {
                    TokenType::INT(ch.to_digit(10).unwrap().to_owned().try_into().unwrap())
                }
                Some('+') => TokenType::ADD,
                Some('-') => TokenType::MIN,
                Some('*') => TokenType::MULT,
                Some('/') => TokenType::DIV,
                Some(' ') => TokenType::WS,
                None => TokenType::EOF,
                _ => TokenType::UNKNOWN,
            }
        }

        fn produce_token(&self, start: usize, end: usize) {
            let mut tokens = self.tokens.borrow_mut();
            tokens.push(&self.source[start..end]);
        }

        fn get_char_at(&self, position: usize) -> Option<char> {
            if position >= self.source.len() {
                return None;
            }
            self.source[position..position + 1].chars().next()
        }

        fn scan_number(&self, digit_position: usize) -> usize {
            let mut offset = 0;
            while let TokenType::INT(_) =
                Scanner::recognize(self.get_char_at(digit_position + offset))
            {
                offset += 1;
            }
            offset
        }

        pub fn tokenize(&self) -> () {
            while !self.is_eof() {
                let symbol = self.get_char_at(self.current_position.get());

                match Scanner::recognize(symbol) {
                    TokenType::UNKNOWN => {
                        panic!("The character {:?} is not recognized", symbol.unwrap());
                    }

                    TokenType::ADD | TokenType::DIV | TokenType::MIN | TokenType::MULT => {
                        self.produce_token(
                            self.current_position.get(),
                            self.current_position.get() + 1,
                        );
                        self.current_position.set(self.current_position.get() + 1);
                    }

                    TokenType::INT(_) => {
                        let start_position = self.current_position.get();
                        let offset_end_number = self.scan_number(start_position);
                        self.current_position
                            .set(self.current_position.get() + offset_end_number);
                        self.produce_token(start_position, self.current_position.get());
                    }

                    TokenType::WS => {
                        self.current_position.set(self.current_position.get() + 1);
                    }

                    TokenType::EOF => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::lexer::Scanner;

    #[test]
    fn tokenize() {
        let source_input = "1500+89 / 6 -9*45  ";
        let scanner = Scanner::new(source_input);
        scanner.tokenize();
        let expected_result = vec!["1500", "+", "89", "/", "6", "-", "9", "*", "45"];
        assert_eq!(scanner.tokens.into_inner(), expected_result);
    }
}
