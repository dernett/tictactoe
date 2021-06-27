use std::io;
use std::io::Write;

use tictactoe::game::{Player, Square, TicTacToe};

fn main() {
    let mut game = TicTacToe::new(3);
    loop {
        println!("Computer's turn...");
        if let (_, Some((r, c))) = game.negamax(Player::X, -2, 2) {
            game.board[r][c] = Square::from(Player::X);
            println!("{}", game);
            if game.is_winner(Player::X) {
                println!("Computer won!");
                break;
            } else if game.is_draw() {
                println!("Game was a draw.");
                break;
            }
        }

        let (r, c): (usize, usize) = loop {
            print!("Choose an empty square from 1-{}: ", game.dim * game.dim);
            io::stdout().flush().expect("Failed to flush stdout");
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            match line.trim().parse::<usize>() {
                Ok(num) => {
                    if 1 <= num && num <= game.dim * game.dim {
                        let (r, c) = ((num - 1) / game.dim, (num - 1) % game.dim);
                        if game.board[r][c] != Square::Empty {
                            println!("Error: That square already has a value.");
                            continue;
                        }
                        break (r, c);
                    } else {
                        println!("Error: Enter a valid number.");
                        continue;
                    }
                }
                Err(_) => {
                    println!("Error: Enter a valid number.");
                    continue;
                }
            }
        };

        game.board[r][c] = Square::from(Player::O);
        println!("{}", game);
        if game.is_winner(Player::O) {
            println!("You... won?");
            break;
        } else if game.is_draw() {
            println!("Game was a draw.");
            break;
        }
    }
}
