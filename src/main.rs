use std::cmp;
use std::fmt;
use std::io;
use std::io::Write;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    O,
    X,
}

impl Player {
    fn opponent(self) -> Player {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

type Move = (usize, usize);

struct TicTacToe {
    board: Vec<Vec<Option<Player>>>,
    dim: usize,
}

impl TicTacToe {
    fn new(dim: usize) -> TicTacToe {
        TicTacToe {
            board: vec![vec![None; dim]; dim],
            dim,
        }
    }

    fn is_winner(&self, p: Player) -> bool {
        let (mut row, mut col) = (false, false);
        let (mut diagl, mut diagr) = (true, true);

        for i in 0..self.dim {
            row |= (0..self.dim).all(|j| self.board[i][j] == Some(p));
            col |= (0..self.dim).all(|j| self.board[j][i] == Some(p));
            diagl &= self.board[i][self.dim - 1 - i] == Some(p);
            diagr &= self.board[i][i] == Some(p);
        }

        row | col | diagl | diagr
    }

    fn is_draw(&self) -> bool {
        self.board.iter().flatten().all(|x| x.is_some())
    }

    fn negamax(&mut self, p: Player, mut alpha: i32, beta: i32) -> (i32, Option<Move>) {
        if self.is_winner(p) {
            return (1, None);
        } else if self.is_winner(p.opponent()) {
            return (-1, None);
        } else if self.is_draw() {
            return (0, None);
        }

        let mut ret = (i32::MIN, None);

        for r in 0..self.dim {
            for c in 0..self.dim {
                if self.board[r][c].is_some() {
                    continue;
                }

                self.board[r][c] = Some(p);
                let (v, _) = self.negamax(p.opponent(), alpha, beta);
                self.board[r][c] = None;

                ret = cmp::max(ret, (-v, Some((r, c))));
                alpha = cmp::max(alpha, ret.0);
                if alpha >= beta {
                    return ret;
                }
            }
        }

        ret
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::O => write!(f, "O"),
            Player::X => write!(f, "X"),
        }
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.dim {
            for c in 0..self.dim {
                if let Some(p) = self.board[r][c] {
                    write!(f, "{}", p);
                } else {
                    write!(f, ".");
                }
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn main() {
    let mut game = TicTacToe::new(3);
    loop {
        println!("Computer's turn...");
        if let (_, Some((r, c))) = game.negamax(Player::X, -2, 2) {
            game.board[r][c] = Some(Player::X);
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
                        if game.board[r][c].is_some() {
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

        game.board[r][c] = Some(Player::O);
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
