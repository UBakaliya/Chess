# Rust Chess Game

Welcome to my Rust-based chess game project! This is a console-based chess game implemented in Rust. The project aims to provide a simple yet functional chess-playing experience.

## Features

- **Console-based Interface**: Play chess directly in your terminal.
- **Standard Chess Rules**: Follows the standard rules of chess.
- **Two-player Mode**: Play against a friend locally.

## Getting Started

### Prerequisites

To run this chess game, you'll need:

- Rust installed on your machine. If you haven't installed Rust, you can get it from [here](https://www.rust-lang.org/tools/install).

### Installation

1. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/UBakaliya/chess.git
    ```

2. Navigate into the project directory:

    ```bash
    cd chess
    ```

3. Build and run the game:

    ```bash
    cargo run
    ```

## Usage

Once the game is running, follow the on-screen prompts to make your moves. The board will be displayed in the console, and you can input your moves using algebraic notation (e.g., "e2e4" to move a piece from e2 to e4).

## License

This project is licensed under the [MIT License](LICENSE).

---

# Chess

Chess Game in Rust

    src
        - main.rs
        - game
            - set_up.rs
            - display.rs
            - check_moves.rs
            - print.rs
        - tests:
            - tests.rs

    -------------------------------
    main.rs : Get input from the user and have moves

    set_up: Will build table set up everything that needs
                the user to see (chess table)
                - build_table()
                - reset_table()
                - start_game()
                - pick_random_player()
                - move_piece(fromX,fromY, toX, toY)  /// update the table and the piece on the table

    display.rs: will display the items:
                - update_table()
                - display_default_table()
                - current_player()
                - invalid_move()

    check_moves.rs: check moves of the users when they call move_piece()
                - get_piece(fromX, fromY)
                - can_move()
                    - use helper function depending on the piece move the item

    Menu:
        I  = Instructions
        P  = Play
        D  = Display Table
        R  = Reset
        Q  = Quit



