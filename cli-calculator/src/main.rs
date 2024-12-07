use inline_colorization::{color_red, color_reset};
use std::cmp::PartialEq;
use std::env::args;
use unicode_segmentation::UnicodeSegmentation;

const NORMAL_DECIMAL_DELINEATOR: char = '.';
const WEIRD_EURO_DECIMAL_DELINEATOR: char = ',';

#[derive(Debug)]
struct ParseError();

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
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
            _ => Err(ParseError()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Operator { operator: Operator },
    Operand { parsed_value: f32 },
}

impl Token {
    fn to_operand(&self) -> Operand {
        match self {
            Token::Operator { .. } => {
                panic!();
            }
            Token::Operand { parsed_value, .. } => Operand(*parsed_value),
        }
    }
}

#[derive(Debug)]
struct Operand(f32);

impl Operand {
    fn to_value(&self) -> f32 {
        self.0
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
            _ => panic!("Unsupported unary operator"),
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

fn main() {
    let expression_opt = args().nth(1);

    match expression_opt {
        Some(expression) => {
            let tokens: Vec<Token> = tokenize(&expression);
            let is_valid = validate_tokens(&tokens);

            if !is_valid {
                panic!("Invalid expression: {}", expression);
            }

            let result = evaluate(tokens);
            println!("The result of is {:?}", result);
        }
        None => {
            println!("{color_red}No expression provided{color_reset}");
            println!();
        }
    }
}

fn tokenize(input: &String) -> Vec<Token> {
    let input_length = input.graphemes(true).count() as i32;
    let mut tokens: Vec<Token> = vec![];
    let mut index: i32 = 0;
    let mut raw_value: String = String::new();

    for letter in input.chars() {
        if letter.is_digit(10)
            || letter == NORMAL_DECIMAL_DELINEATOR
            || letter == WEIRD_EURO_DECIMAL_DELINEATOR
        {
            raw_value = format!("{}{}", raw_value, letter);
        } else if letter.is_whitespace() || index == input_length {
            append_if_operand(&mut tokens, &mut raw_value);
        } else {
            // stash the parsed operand
            append_if_operand(&mut tokens, &mut raw_value);

            // this should be an operator
            let operator = Operator::parse(letter).unwrap();
            tokens.push(Token::Operator { operator });
        }

        index += 1;
    }

    tokens
}

fn append_if_operand(tokens: &mut Vec<Token>, raw_value: &mut String) {
    let token_opt = evaluate_operand(&raw_value);
    match token_opt {
        Some(token) => {
            tokens.push(token);
            raw_value.clear();
        }
        None => {}
    }
}

fn evaluate_operand(raw_value: &String) -> Option<Token> {
    if !raw_value.is_empty() {
        let token = Token::Operand {
            parsed_value: normalize_numeric_string(raw_value).parse::<f32>().unwrap(),
        };

        return Some(token);
    }

    None
}

fn normalize_numeric_string(value: &String) -> String {
    value
        .chars()
        .filter(|&c| c != '.' && c != ' ')
        .map(|c| if c == ',' { '.' } else { c })
        .collect::<String>()
}

fn validate_tokens(tokens: &Vec<Token>) -> bool {
    let last_index = tokens.len() - 1;

    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Operator { operator, .. } => {
                if *operator == Operator::Divide || *operator == Operator::Multiply {
                    // we cannot start an expression with a multiply or divide operator
                    if index == 0 {
                        return false;
                    } else {
                        let previous_token = &tokens[index - 1];
                        match previous_token {
                            Token::Operator { .. } => {
                                return false;
                            }
                            Token::Operand { .. } => {}
                        }
                    }
                }

                // we cannot end an expression with an operator
                if index == last_index {
                    return false;
                }
            }
            Token::Operand { .. } => {
                if index != 0 {
                    let previous_token = &tokens[index - 1];
                    match previous_token {
                        Token::Operator { .. } => {}
                        Token::Operand { .. } => {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
}

fn evaluate(tokens: Vec<Token>) -> f32 {
    let mut initial_pass_result: Vec<Token> = Vec::new();
    let initial_last_index = tokens.len() - 1;
    let mut initial_loop_cursor = 0;

    while initial_loop_cursor <= initial_last_index {
        let token = tokens.iter().nth(initial_loop_cursor).unwrap();

        match token {
            Token::Operator { operator, .. } => {
                if *operator == Operator::Multiply || *operator == Operator::Divide {
                    let previous_operand = initial_pass_result.pop().unwrap().to_operand();
                    let next_operand = tokens
                        .iter()
                        .nth(initial_last_index + 1)
                        .unwrap()
                        .to_operand();

                    let operation = BinaryOperation {
                        left_hand: previous_operand,
                        operator: *operator,
                        right_hand: next_operand,
                    };

                    initial_pass_result.push(Token::Operand {
                        parsed_value: operation.evaluate(),
                    });
                    initial_loop_cursor += 2;
                } else {
                    initial_pass_result.push(*token);
                    initial_loop_cursor += 1;
                }
            }
            Token::Operand { .. } => {
                initial_pass_result.push(*token);
                initial_loop_cursor += 1;
            }
        }
    }

    let mut result = 0f32;

    let final_last_index = initial_pass_result.len() - 1;
    let mut final_loop_cursor = 0;

    while final_loop_cursor <= final_last_index {
        let token = &initial_pass_result[initial_loop_cursor];

        match token {
            Token::Operator { operator, .. } => {
                let next_token = &initial_pass_result[initial_loop_cursor + 1];
                let operation = UnaryOperation {
                    operator: *operator,
                    operand: next_token.to_operand(),
                };
                result += operation.evaluate();
                final_loop_cursor += 2;
            }
            Token::Operand { parsed_value, .. } => {
                result += parsed_value;
                final_loop_cursor += 1;
            }
        }
    }

    result
}
