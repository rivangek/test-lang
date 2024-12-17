#[derive(Debug)]
pub enum KeywordType {
    Local,
    Function,
    End
}

/// Type for proto-tokens, basically tokens whose type is still capturing and need a type of processor.
#[derive(Clone, Copy)]
enum TokenProcessor {
    Malformed,
    Number,
    Identifier,
    String,
    Break, /// Break processor that literaly means skip.
    Operator
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(i32),
    Keyword(KeywordType),
    Assign,
    LeftParenthesis,
    RightParenthesis,
    UnexpectedSymbol(String)
}

pub struct Tokenizer {
    content: String,

    len: usize,
    current: usize,

    last_processor: TokenProcessor,
    current_processor: TokenProcessor,
}

fn is_string_token(c: &char) -> bool {
    (c == &'"') | (c == &'\'')
}

impl Tokenizer {
    pub fn new(content: String) -> Self {
        let len = content.len();

        Self {
            content,
            len,

            current: 0,
            last_processor: TokenProcessor::Malformed,
            current_processor: TokenProcessor::Malformed
        }
    }

    pub fn capture(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current < self.len {
            let captured_token = self.capture_token();

            if let Some(token) = captured_token {
                if let Token::UnexpectedSymbol(symbol) = &token {
                    //This isn't tokenizer business
                    //panic!("Unexpected symbol \"{}\" at {}th index", symbol, self.current);
                }

                tokens.push(token);
            }
        }

        tokens
    }

    fn change_processor(&mut self, new: TokenProcessor) {
        self.last_processor = self.current_processor;
        self.current_processor = new;
    }

    fn is_processing_identifier(&self) -> bool {
        if let TokenProcessor::Identifier = self.current_processor {
            return true;
        }

        false
    }

    fn is_processing_string(&self) -> bool {
        if let TokenProcessor::String = self.current_processor {
            return true;
        }

        false
    }

    fn capture_token(&mut self) -> Option<Token> {
        let chars: Vec<char> = self.content
            .chars()
            .collect();

        let mut token = None;
        let mut formed_string = String::new();

        while self.current < self.len {
            let current_character = chars[self.current];
            let next_character = chars.get(self.current + 1);

            self.current += 1;

            // Choose a token processor

            // Identifier
            if current_character.is_alphabetic() {
                if !self.is_processing_string() {
                    self.change_processor(TokenProcessor::Identifier);
                }
            }

            // Break
            if current_character.is_whitespace() && !self.is_processing_string() {
                self.change_processor(TokenProcessor::Break);
            }

            // String & operator
            if current_character.is_ascii_punctuation() {
                if is_string_token(&current_character) {
                    self.change_processor(TokenProcessor::String);
                } else { // Operator case

                    // If it isn't processing string proceed. (Prevents symbols in string to be considered as tokens)
                    if !self.is_processing_string() {
                        self.change_processor(TokenProcessor::Operator);
                    }

                }
            }

            if current_character.is_numeric() {
                if !self.is_processing_string() && !self.is_processing_identifier() {
                    self.change_processor(TokenProcessor::Number);
                }
            }

            // Match the current processor to implement its logic

            match &self.current_processor {
                TokenProcessor::Break => {
                    break;
                }

                TokenProcessor::Operator => {
                    token = match current_character {
                        '=' => Some(Token::Assign),
                        '(' => Some(Token::LeftParenthesis),
                        ')' => Some(Token::RightParenthesis),
                        _ => Some(Token::UnexpectedSymbol(current_character.to_string()))
                    };

                    break
                }

                TokenProcessor::String => {
                    // Same logic as identifier but includes double quotes 
                    // and doesn't match for keywords.
                    if !is_string_token(&current_character) {
                        formed_string.push(current_character);
                    }

                    token = Some(Token::StringLiteral(formed_string.clone()))
                }

                TokenProcessor::Identifier => {
                    // This won't ignore numbers if the first character starts as an alphabetic.
                    // Only will stop when a separator is found using the next character.
                    formed_string.push(current_character);

                    token = match formed_string.as_str() {
                        "local" => Some(Token::Keyword(KeywordType::Local)),
                        "function" => Some(Token::Keyword(KeywordType::Function)),
                        "end" => Some(Token::Keyword(KeywordType::End)),
                        _ => Some(Token::Identifier(formed_string.clone()))
                    };
                }

                TokenProcessor::Number => {
                    formed_string.push(current_character);
                    token = Some(Token::NumberLiteral(formed_string.parse().unwrap()))
                }

                TokenProcessor::Malformed => formed_string.push(current_character)
            }

            // Check if the look-ahead character is a valid separator to break the current loop
            // so the current token gets completed. Applies to all types of tokens.

            if let Some(character) = next_character {
                if !self.is_processing_string() {
                    if character.is_whitespace() {
                        break;
                    }
    
                    if character.is_ascii_punctuation() {
                        break;
                    }
                }

                // Allow string literal to form and skip the next character (which is the end of string literal).
                if is_string_token(character) {
                    self.current += 1; // Skip next character which is the next " that can start a new literal forming.
                    break;
                }
            }
        }

        self.last_processor = TokenProcessor::Malformed;
        self.current_processor = TokenProcessor::Malformed;

        token
    }
}