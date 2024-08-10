use std::io::stdin;
use rand::{Rng};

fn main() {
    let mut total_runs: u32 = 0;
    let mut correct_guesses: u32 = 0;
    let mut loop_guard = true;
    let mut rng = rand::thread_rng();
    let max_range = 10;
    
    println!();
    println!("WELCOME TO TEXT GUESSR");

    while loop_guard {
        println!();
        println!("Guess an integer between 0 and {}. If you guess correctly, you get a point.", max_range);
        println!("If you are done playing, type 'exit' and get your results.");
        println!();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let sanitized_input = input.trim();

        if sanitized_input == "exit" {
            println!("Out of {} runs, you guessed {} successfully.", total_runs, correct_guesses);
            loop_guard = false;
        } else {
            let parse_result = sanitized_input.parse::<i32>();
            
            match parse_result { 
                Ok(value) => {
                    if value < 0 || value > max_range {
                        println!("Invalid value");
                    } else { 
                        let random: i32 = rng.gen_range(0..max_range + 1);
                        total_runs += 1;
                        
                        if random == value {
                            correct_guesses += 1;
                            println!("Wow, you are good at this. You guessed right");
                        } else { 
                            println!("I guess you aren't good at this. The correct value is {}", random);
                        }
                    } 
                },
                Err(_error) => println!("Invalid value")
            }
        }
    }
}
