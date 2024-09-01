use std::env::args;
use inline_colorization::{color_red, color_reset};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
struct ParseError(String);

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Token {
    Operator {
        index: i32,
        operator: Operator,
    },
    Operand {
        start_index: i32,
        end_index: i32,
        raw_value: String,
    },
}

impl Operator {
    fn parse(value: char) -> Result<Operator, ParseError> {
        match value {
            '*' => Ok(Operator::Multiply),
            'x' => Ok(Operator::Multiply),
            '÷' => Ok(Operator::Divide),
            '/' => Ok(Operator::Divide),
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Subtract),
            _ => Err(ParseError(format!("Invalid operator provided: '{}'", value)))
        }
    }
}

fn main() {
    let expression_opt = args().nth(1);

    match expression_opt {
        Some(expression) => {
            let tokens: Vec<Token> = tokenize(expression);
            println!("{:?}", tokens);
        }
        None => {
            println!("{color_red}No expression provided{color_reset}");
            println!();
        }
    }
}

fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec!();
    let mut index: i32 = 0;
    let mut raw_value: String = String::new();

    for letter in input.chars() {
        if (letter.is_digit(10) || letter == '.' || letter == ',') {
            raw_value = format!("{}{}", raw_value, letter);
        } else if letter.is_whitespace() {
            if !raw_value.is_empty() {
                tokens.push(Token::Operand {
                    end_index: index,
                    start_index: index - raw_value.graphemes(true).count() as i32,
                    raw_value: raw_value.clone(),
                });
                // reset the value read out
                raw_value = String::new();
            }
        } else if index == input.graphemes(true).count() as i32 {
            if !raw_value.is_empty() {
                tokens.push(Token::Operand {
                    end_index: index,
                    start_index: index - raw_value.graphemes(true).count() as i32,
                    raw_value: raw_value.clone(),
                });
                // reset the value read out
                raw_value = String::new();
            }
        }  else {
            // stash the parsed operand
            if !raw_value.is_empty() {
                tokens.push(Token::Operand {
                    end_index: index,
                    start_index: index - raw_value.graphemes(true).count() as i32,
                    raw_value: raw_value.clone(),
                });
                // reset the value read out
                raw_value = String::new();
            }

            // this should be an operator
            let operator = Operator::parse(letter).unwrap();
            tokens.push(Token::Operator {
                index,
                operator
            });
        }

        index += 1;
    }
    
    return tokens;
}
