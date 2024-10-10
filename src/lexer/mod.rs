#[derive(Debug)]
pub enum Token {
    // Keywords
    If,
    Else,

    // Literals/Identifiers
    IntLiteral(i64),
    Identifier(String),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,

    // End of file
    EOF,
}

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }
    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();

        match self.input.next() {
            Some(ch) => match ch {
                '0'..='9' => self.lex_number(ch),
                'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier_or_keyword(ch),
                '+' => Ok(Token::Plus),
                '-' => Ok(Token::Minus),
                '*' => Ok(Token::Multiply),
                '/' => Ok(Token::Divide),
                '=' => self.lex_equals_or_assign(),
                '<' => self.lex_less_than(),
                '>' => self.lex_greater_than(),
                '(' => Ok(Token::LeftParen),
                ')' => Ok(Token::RightParen),
                '{' => Ok(Token::LeftBrace),
                '}' => Ok(Token::RightBrace),
                ';' => Ok(Token::Semicolon),
                ',' => Ok(Token::Comma),
                '!' => self.lex_not_equal(),
                _ => Err(format!("Unexpected character: {}", ch)),
            },
            None => Ok(Token::EOF),
        }
    }

    fn lex_number(&mut self, first_digit: char) -> Result<Token, String> {
        let mut number = first_digit.to_string();

        while let Some(ch) = self.input.peek() {
            if ch.is_ascii_digit() {
                number.push(self.input.next().unwrap());
            } else {
                break;
            }
        }

        match number.parse::<i64>() {
            Ok(n) => Ok(Token::IntLiteral(n)),
            Err(_) => Err(format!("Not an integer {}", number)),
        }
    }

    fn lex_identifier_or_keyword(&mut self, first_char: char) -> Result<Token, String> {
        let mut name = first_char.to_string();

        while let Some(ch) = self.input.peek() {
            if ch.is_ascii_alphanumeric() || ch == &'_' {
                name.push(self.input.next().unwrap());
            } else if ch.is_whitespace() || ch == &';' {
                break;
            } else {
                return Err(format!("Unexpected char {}", ch));
            }
        }

        match name.as_str() {
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            _ => Ok(Token::Identifier(name)),
        }
    }

    fn lex_equals_or_assign(&mut self) -> Result<Token, String> {
        if self.input.peek() == Some(&'=') {
            self.input.next();
            Ok(Token::Equal)
        } else {
            Ok(Token::Assign)
        }
    }

    fn lex_less_than(&mut self) -> Result<Token, String> {
        if self.input.peek() == Some(&'=') {
            self.input.next();
            Ok(Token::LessThanOrEqual)
        } else {
            Ok(Token::LessThan)
        }
    }

    fn lex_greater_than(&mut self) -> Result<Token, String> {
        if self.input.peek() == Some(&'=') {
            self.input.next();
            Ok(Token::GreaterThanOrEqual)
        } else {
            Ok(Token::GreaterThan)
        }
    }

    fn lex_not_equal(&mut self) -> Result<Token, String> {
        if self.input.peek() == Some(&'=') {
            self.input.next();
            Ok(Token::NotEqual)
        } else {
            Err("Expected '=' after '!'".to_string())
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else if ch == &'#' {
                self.input.next();
                while let Some(ch) = self.input.next() {
                    if ch == '\n' || ch == '#' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(Token::EOF) => break,
            Ok(t) => tokens.push(t),
            Err(e) => return Err(e),
        }
    }

    Ok(tokens)
}
