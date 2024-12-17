#[derive(Debug)]
enum KeywordType {
    Local
}

/// Type for proto-tokens, basically tokens whose type is still capturing and need a type of processor.
#[derive(Clone, Copy)]
enum TokenProcessor {
    Malformed,
    Number,
    Text,
    Separator,
    Operator
}

#[derive(Debug)]
pub enum Token {
    Quote,
    Identifier(String),
    NumberLiteral(i32),
    Keyword(KeywordType),
    Assign,
}

pub struct Tokenizer {
    content: String,

    len: usize,
    current: usize,

    last_processor: TokenProcessor,
    current_processor: TokenProcessor,
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
                tokens.push(token);
            }
        }

        tokens
    }

    fn change_processor(&mut self, new: TokenProcessor) {
        self.last_processor = self.current_processor;
        self.current_processor = new;
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

            if current_character.is_alphabetic() {
                self.change_processor(TokenProcessor::Text);
            }

            if current_character.is_whitespace() {
                self.change_processor(TokenProcessor::Separator);
            }

            if current_character.is_ascii_punctuation() {
                self.change_processor(TokenProcessor::Operator);
            }

            if current_character.is_numeric() {
                let processing_text = if let TokenProcessor::Text = self.current_processor {
                    true
                } else {
                    false
                };

                if !processing_text {
                    self.change_processor(TokenProcessor::Number);
                }
            }

            // Match the current processor to implement its logic

            match &self.current_processor {
                TokenProcessor::Separator => {
                    break;
                }

                TokenProcessor::Operator => {
                    token = match current_character {
                        '=' => Some(Token::Assign),
                        _ => None
                    };

                    break;
                }

                TokenProcessor::Text => {
                    // This won't ignore numbers if the first character starts as an alphabetic.
                    // Only will stop when a separator is found using the next character.
                    formed_string.push(current_character);

                    token = match formed_string.as_str() {
                        "local" => Some(Token::Keyword(KeywordType::Local)),
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
                if character.is_whitespace() {
                    break;
                }
            }
        }

        self.last_processor = TokenProcessor::Malformed;
        self.current_processor = TokenProcessor::Malformed;

        token
    }
}