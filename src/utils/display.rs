use crate::{types::*, utils::game::get_piece};

pub fn board(board: Board) {
    let mut counter: i32 = 8;

    for row in board {
        for ele in row {
            match ele.is_occupied() {
                false => print!("\t\x1b[31;1m.\x1b[0m"),
                _ => print!("\t{}", get_piece(ele).unwrap()),
            }
        }
        println!("  | {}\n", counter);
        counter -= 1;
    }

    println!("\t___________________________________________________________");
    println!("\ta\tb\tc\td\te\tf\tg\th");
}

pub fn players_pieces(player1: i32) {
    match player1 {
        1 => println!("Player 1: BLACK and player 2: WHITE"),
        _ => println!("Player 1: WHITE and player 2: BLACK"),
    }
}

pub fn invalid_choice_message() {
    println!("\t\x1b[31;1mInvalid choice\x1b[0m");
}

pub fn instructions() {
    println!("Welcome to Rust Chess!");
    println!("Instructions:");
    println!("\t1. Each player takes turns to make a move.");
    println!("\t2. Input your move using algebraic notation (e.g., 'e2e4').");
    println!("\t3. The game will display the board after each move.");
    println!("\t4. Capturing is done by moving onto the square occupied by the opponent's piece.");
    println!("\t5. The game ends when a player is in checkmate or a stalemate occurs.");
    println!("\t6. While playing If you want to Reset or Quit the game you should write 'RESET or QUIT'.");
    println!("\t6. Enjoy playing!");
}

pub fn menu() {
    print!("\n\n\nMenu:\n \tI = Instructions\n \tP = Play\n \tD = Display Board\n \tR = Reset\n \tQ = Quit\n");
    println!("You choose >> ");
}

pub fn quite_game() {
    println!("Exiting...");
    std::process::exit(0);
}
