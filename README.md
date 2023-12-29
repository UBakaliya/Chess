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
