use std::io::stdin;

fn main() {
    let mut total_runs: u32 = 0;
    let mut correct_guesses: u32 = 0;
    let mut loop_guard = true;

    while loop_guard {
        total_runs += 1;
        println!("Guess an integer between 0 and 20. If you guess correctly, you get a point:");

        let mut input = String::new();
        let n = stdin().read_line(&mut input).unwrap();
        println!("Bytes read {}", n);
        println!("Input read {}", input);

        if input == "exit" {
            println!("Out of {} runs, you guessed {} successfully", total_runs, correct_guesses);
            loop_guard = false;
        } else {
            print!("Hello");
        }
    }
}
