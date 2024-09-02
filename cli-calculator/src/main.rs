use std::env::args;
use inline_colorization::{color_red, color_reset};
use unicode_segmentation::UnicodeSegmentation;

const NORMAL_DECIMAL_DELINEATOR: char = '.';
const WEIRD_EURO_DECIMAL_DELINEATOR: char = ',';

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
    let input_length = input.graphemes(true).count() as i32;
    let mut tokens: Vec<Token> = vec!();
    let mut index: i32 = 0;
    let mut raw_value: String = String::new();

    for letter in input.chars() {
        if letter.is_digit(10) || letter == NORMAL_DECIMAL_DELINEATOR || letter == WEIRD_EURO_DECIMAL_DELINEATOR {
            raw_value = format!("{}{}", raw_value, letter);
        } else if letter.is_whitespace() || index == input_length {
            append_if_operand(&mut tokens, index, &mut raw_value);
        } else {
            // stash the parsed operand
            append_if_operand(&mut tokens, index, &mut raw_value);

            // this should be an operator
            let operator = Operator::parse(letter).unwrap();
            tokens.push(Token::Operator {
                index,
                operator,
            });
        }

        index += 1;
    }

    tokens
}

fn append_if_operand(mut tokens: &mut Vec<Token>, index: i32, raw_value: &mut String) {
    let token_opt = evaluate_operand(index, &raw_value);
    match token_opt {
        Some(token) => {
            tokens.push(token);
            raw_value.clear();
        }
        None => {}
    }
}

fn evaluate_operand(index: i32, raw_value: &String) -> Option<Token> {
    if !raw_value.is_empty() {
        let token = Token::Operand {
            end_index: index,
            start_index: index - raw_value.graphemes(true).count() as i32,
            raw_value: raw_value.clone(),
        };

        return Some(token);
    }

    None
}
