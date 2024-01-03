mod utils;
use utils::*;

fn main() {
    // display::menu();

    // let mut input = String::new();
    // loop {
    //     input.clear();
    //     if let Err(err) = std::io::stdin().read_line(&mut input) {
    //         eprintln!("Error reading input: {}", err);
    //         continue;
    //     }

    //     // Trim leading/trailing whitespace for better handling
    //     let trimmed_input = input.trim();

    //     if trimmed_input.is_empty() {
    //         println!("Please enter a valid option.");
    //         continue;
    //     }

    //     let choice = trimmed_input.chars().next().unwrap().to_ascii_uppercase();

    //     match choice {
    //         'I' => display::instructions(),
    //         'Q' => display::quite_game(),
    //         'P' => game::play(),
    //         _ => display::invalid_choice_message(),
    //     };

    //     display::menu();
    // }
}
