use std::cmp::PartialEq;
use std::env::args;
use inline_colorization::{color_red, color_reset};
use unicode_segmentation::UnicodeSegmentation;

const NORMAL_DECIMAL_DELINEATOR: char = '.';
const WEIRD_EURO_DECIMAL_DELINEATOR: char = ',';

#[derive(Debug, Clone)]
struct ParseError(String);

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
enum Operand {
    Value(f32),
    Operation(dyn Operation),
}

impl Operand {
    fn to_value(&self) -> f32 {
        match &self {
            Operand::Value(value) => value.to_owned(),
            Operand::Operation(operation) => operation.evaluate()
        }
    }
}

#[derive(Debug)]
struct UnaryOperation {
    operator: Operator,
    operand: Operand,
}

#[derive(Debug)]
struct BinaryOperation {
    left_hand: Operand,
    operator: Operator,
    right_hand: Operand,
}

trait Operation {
    fn evaluate(&self) -> f32;
}

impl Operation for UnaryOperation {
    fn evaluate(&self) -> f32 {
        let evaluated_operand = self.operand.to_value();

        match self.operator {
            Operator::Add => evaluated_operand,
            Operator::Subtract => 0f32 - evaluated_operand,
            _ => panic!("Unsupported unary iperator")
        }
    }
}

impl Operation for BinaryOperation {
    fn evaluate(&self) -> f32 {
        let evaluated_left = self.left_hand.to_value();
        let evaluated_right = self.right_hand.to_value();

        match self.operator {
            Operator::Add => evaluated_left + evaluated_right,
            Operator::Subtract => evaluated_left - evaluated_right,
            Operator::Multiply => evaluated_left * evaluated_right,
            Operator::Divide => evaluated_left / evaluated_right,
        }
    }
}

impl PartialEq<Operator> for &Operator {
    fn eq(&self, other: &Operator) -> bool {
        *self == *other
    }
}

fn main() {
    let expression_opt = args().nth(1);

    match expression_opt {
        Some(expression) => {
            let tokens: Vec<Token> = tokenize(expression);
            let is_valid = validate_tokens(&tokens);

            if !is_valid {
                panic!("Invalid expression: {}", expression);
            }

            let operation = to_operation(tokens);
            println!("The result of is {:?}", operation.evaluate());
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

fn normalize_numeric_string(mut value: &String) -> String {
    value.chars()
        .filter(|&c| c != '.' && c != ' ')
        .map(|c| if c == ',' { '.' } else { c })
        .collect::<String>()
}

fn validate_tokens(tokens: &Vec<Token>) -> bool {
    let last_index = tokens.len() -1;

    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Operator { operator, .. } => {
                if operator == Operator::Divide || operator == Operator::Multiply {
                    // we cannot start an expression with a multiply or divide operator
                    if index == 0  {
                        return false;
                    } else {
                        let previous_token = &tokens[index - 1];
                        match previous_token {
                            Token::Operator {..} => {
                                return false;
                            }
                            Token::Operand {..} => {}
                        }
                    }
                }

                // we cannot end an expression with an operator
                if index ==  last_index{
                    return false;
                }
            }
            Token::Operand { .. } => {
                let previous_token = &tokens[index - 1];
                match previous_token {
                    Token::Operator {..} => {}
                    Token::Operand {..} => {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn to_operation(tokens: Vec<Token>) -> Box<dyn Operation> {
    let first_token = tokens.iter().nth(0).unwrap();
    match first_token {
        Token::Operator {operator, index} => {
            if Operator::Add == *operator || *operator == Operator::Subtract {
                let second_token = tokens.iter().nth(1).unwrap();
                match *second_token {
                    Token::Operator {sec_operator, sec_index} => {

                    }
                }
            } else {
                panic!("Unsupported operator");
            }
        }
    }
}
