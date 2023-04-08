use crate::input::Input;

pub enum TokenType {
    Name,
    Function,
    Return,

    Const,

    Dot,
    Comma,
    Semicolon,

    Equal,
    Plus,

    SingleQuote,

    RoundBraceOpen,
    RoundBraceClose,
    CurlyBraceOpen,
    CurlyBraceClose,

    String,
    Int,
}

pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
    pub column: u32,
    pub content: String,
}

pub struct Lexer {
    pub input: Input,
    pub cursor: u32,
    pub line: u32,
    pub column: u32,
}

impl TokenType {
    fn to_string(&self) -> &str {
        match self {
            TokenType::Name => "Name",
            TokenType::Function => "Function",
            TokenType::Return => "Return",

            TokenType::Const => "Const",

            TokenType::Dot => "Dot",
            TokenType::Comma => "Comma",
            TokenType::Semicolon => "Semicolon",

            TokenType::Equal => "Equal",
            TokenType::Plus => "Plus",

            TokenType::SingleQuote => "SingleQuote",

            TokenType::RoundBraceOpen => "RoundBraceOpen",
            TokenType::RoundBraceClose => "RoundBraceClose",
            TokenType::CurlyBraceOpen => "CurlyBraceOpen",
            TokenType::CurlyBraceClose => "CurlyBraceClose",

            TokenType::String => "String",
            TokenType::Int => "Int",
        }
    }
}

impl Lexer {

    // constructor
    pub fn new(input: Input) -> Lexer {
        Lexer {
            input: input,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    // external helper
    pub fn pretty_format_token(&self, token: &Token) -> String {
        format!(
            "{}:{}:{} {} \"{}\"",
            self.input.file,
            token.line,
            token.column,
            token.token_type.to_string(),
            token.content,
        )
    }

    // helper
    fn end_reached(&self) -> bool {
        self.cursor + 1 >= self.input.content.len() as u32
    }

    fn get_character(&self, cursor: u32) -> char {
        self.input.content.chars().nth(cursor as usize).unwrap()
    }

    fn increment_cursor(&mut self) {
        self.cursor += 1;
        self.column += 1;

        if self.get_character(self.cursor-1) == '\n' {
            self.column = 1;
            self.line += 1;
        }
    }

    // parsing helper
    fn is_number(&self, character: &char) -> bool {
        let ascii_value: u32 = *character as u32;

        if ascii_value > 47 && ascii_value < 58 {
            return true;
        }

        return false;
    }

    fn is_special_character(&self, character: &char) -> bool {
        let ascii_value: u32 = *character as u32;

        // number
        if self.is_number(character) {
            return false;
        }

        // uppercase letter
        if ascii_value > 64 && ascii_value < 91 {
            return false;
        }

        // underscore
        if ascii_value == 95 {
            return false;
        }

        // lowercase letter
        if ascii_value > 95 && ascii_value < 123 {
            return false;
        }

        return true;
    }

    fn is_quote(&self, character: &char) -> bool {
        character == &'"' || character == &'\''
    }

    fn is_whitespace(&self, character: &char) -> bool {
        character == &' ' || character == &'\t' || character == &'\n'
    }

    // parsing
    fn parse_word(&mut self) -> Token {
        let mut string_content: Vec<char> = Vec::new();
        let mut character: char;
        let start_line: u32 = self.line;
        let start_column: u32 = self.column;

        loop {
            character = self.get_character(self.cursor);

            if self.is_whitespace(&character) || self.is_special_character(&character) {
                break;
            }

            self.increment_cursor();
            string_content.push(character);
        }

        // keywords
        let content: String = String::from_iter(string_content.iter());

        let token_type: TokenType = match content.as_str() {
            "function" => TokenType::Function,
            "return" => TokenType::Return,
            "const" => TokenType::Const,
            _ => TokenType::Name,
        };

        Token {
            token_type: token_type,
            line: start_line,
            column: start_column,
            content: content,
        }
    }

    fn parse_string(&mut self) -> Token {
        let mut string_content: Vec<char> = Vec::new();
        let mut character: char;
        let start_line: u32 = self.line;
        let start_column: u32 = self.column;

        self.increment_cursor();

        loop {
            character = self.get_character(self.cursor);

            if self.is_quote(&character) {
                break;
            }

            self.increment_cursor();
            string_content.push(character);
        }

        self.increment_cursor();

        Token {
            token_type: TokenType::String,
            line: start_line,
            column: start_column,
            content: String::from_iter(string_content.iter()),
        }
    }

    fn parse_number(&mut self) -> Token {
        let mut string_content: Vec<char> = Vec::new();
        let mut character: char;
        let start_line: u32 = self.line;
        let start_column: u32 = self.column;

        loop {
            character = self.get_character(self.cursor);

            if !self.is_number(&character) {
                break;
            }

            self.increment_cursor();
            string_content.push(character);
        }

        Token {
            token_type: TokenType::Int,
            line: start_line,
            column: start_column,
            content: String::from_iter(string_content.iter()),
        }
    }

    fn parse_special_character(&mut self) -> Token {
        let character: char = self.get_character(self.cursor);

        let token_type: TokenType = match character {
            '.' => TokenType::Dot,
            ',' => TokenType::Comma,
            ';' => TokenType::Semicolon,

            '=' => TokenType::Equal,
            '+' => TokenType::Plus,

            '\'' => TokenType::SingleQuote,

            '(' => TokenType::RoundBraceOpen,
            ')' => TokenType::RoundBraceClose,
            '{' => TokenType::CurlyBraceOpen,
            '}' => TokenType::CurlyBraceClose,

            // FIXME
            _ => TokenType::Name,
        };

        let token: Token = Token {
            token_type: token_type,
            line: self.line,
            column: self.column,
            content: String::from(character),
        };

        self.increment_cursor();

        return token;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut character: char;

        // find start
        if self.end_reached() {
            return None
        }

        loop {
            character = self.get_character(self.cursor);

            if !self.is_whitespace(&character) {
                break;
            }

            self.increment_cursor();
        }

        if self.end_reached() {
            return None
        }

        // parse token
        let token: Token;

        if self.is_special_character(&character) {
            if self.is_quote(&character) {
                token = self.parse_string();
            } else {
                token = self.parse_special_character();
            }
        } else {
            if self.is_number(&character) {
                token = self.parse_number();
            } else {
                token = self.parse_word();
            }
        }

        return Some(token);
    }
}
