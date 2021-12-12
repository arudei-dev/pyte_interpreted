pub mod token;

use token::{Token, TyNumber, TyOperator};

use lazy_static::lazy_static;
use regex::Regex;

use crate::shared::errors::{CompileError, ErrorDetail};

lazy_static! {
    static ref RE_ESCAPED: Regex = Regex::new("\\s|\\t").unwrap();
    static ref RE_DIGITS: Regex = Regex::new("\\d").unwrap();
}

pub struct Lexer {
    raw_text: String,
    pos_idx: Option<usize>,
}

impl Lexer {
    pub fn from(raw_text: String) -> Self {
        Self {
            raw_text,
            pos_idx: Some(0),
        }
    }

    fn advance_one(&mut self) {
        self.pos_idx = if let Some(i) = self.pos_idx {
            if i < self.raw_text.len() - 1 {
                Some(i + 1)
            } else {
                None
            }
        } else {
            None
        };
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError> {
        let mut tokenized: Vec<Token> = vec![];

        loop {
            if let Some(i) = self.pos_idx {
                let s = &self.raw_text[i..i + 1];

                match s {
                    "+" => {
                        tokenized.push(Token::Operator(TyOperator::Plus));
                    }
                    "-" => {
                        tokenized.push(Token::Operator(TyOperator::Minus));
                    }
                    "*" => {
                        tokenized.push(Token::Operator(TyOperator::Multiply));
                    }
                    "/" => {
                        tokenized.push(Token::Operator(TyOperator::Division));
                    }
                    "(" => {
                        tokenized.push(Token::Operator(TyOperator::LeftParn));
                    }
                    ")" => {
                        tokenized.push(Token::Operator(TyOperator::RightParn));
                    }
                    s => {
                        if RE_DIGITS.is_match(s) {
                            tokenized.push(self.tokenize_num());
                        } else if RE_ESCAPED.is_match(s) {
                        } else {
                            return Err(CompileError::IllegalChar(ErrorDetail {
                                details: format!("Unknown character found: {}", s),
                                pos_start: i,
                                pos_end: i,
                            }));
                        }
                    }
                }

                self.advance_one();
            } else {
                break;
            }
        }

        return Ok(tokenized);
    }

    fn tokenize_num(&mut self) -> Token {
        let mut raw_num = format!("");
        let mut dot_count = 0;
        let mut is_still_valid = true;

        loop {
            if let Some(i) = self.pos_idx {
                let s = &self.raw_text[i..i + 1];

                if RE_ESCAPED.is_match(s) || (!RE_DIGITS.is_match(s) && s != ".") {
                    break;
                }

                if is_still_valid {
                    if s == "." {
                        if dot_count == 1 {
                            is_still_valid = false;
                            continue;
                        }
                        dot_count += 1;
                        raw_num += ".";
                    } else {
                        raw_num += s;
                    }
                }

                self.advance_one();
            } else {
                break;
            }
        }

        if dot_count == 0 {
            let parsed_num = str::parse::<i64>(&raw_num).unwrap();
            return Token::Number(TyNumber::Int(parsed_num));
        } else {
            let parsed_num = str::parse::<f64>(&raw_num).unwrap();
            return Token::Number(TyNumber::Float(parsed_num));
        }
    }
}
