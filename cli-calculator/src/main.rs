use std::env::args;
use std::error::Error;
use inline_colorization::{color_green, color_red, color_reset, color_cyan, color_yellow};
use unicode_segmentation::UnicodeSegmentation;

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

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
    fn parse(value: char) -> Result<Operator, dyn Error> {
        match value {
            'x' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Subtract),
            _ => Err("Invalid operator")
        }
    }
}

fn main() {
    let expression_opt = args().nth(1);

    match expression_opt {
        Some(expression) => {
            let mut tokens: Vec<Token> = vec!();
            let mut index: i32 = 0;
            let mut raw_value: String = String::new();

            for letter in expression.chars() {
                if letter.is_whitespace() {
                    if !raw_value.is_empty() {
                        tokens.push(Token::Operand {
                            end_index: index,
                            start_index: index - raw_value.graphemes(true).count() as i32,
                            raw_value: raw_value.clone(),
                        });
                    }
                }

                raw_value = format!("{}{}", raw_value, letter);
                index += 1;
            }
        }
        None => {
            println!("{color_red}No expression provided{color_reset}");
            println!();
        }
    }
}
