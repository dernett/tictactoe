use std::cmp;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    O,
    X,
}

impl Player {
    pub fn opponent(self) -> Player {
        match self {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Square {
    Empty,
    Player(Player),
}

impl From<Player> for Square {
    fn from(p: Player) -> Self {
        Square::Player(p)
    }
}

pub struct TicTacToe {
    pub board: Vec<Vec<Square>>,
    pub dim: usize,
}

type Move = (usize, usize);

impl TicTacToe {
    pub fn new(dim: usize) -> TicTacToe {
        TicTacToe {
            board: vec![vec![Square::Empty; dim]; dim],
            dim,
        }
    }

    pub fn is_winner(&self, p: Player) -> bool {
        let (mut row, mut col) = (false, false);
        let (mut diagl, mut diagr) = (true, true);

        for i in 0..self.dim {
            row |= (0..self.dim).all(|j| self.board[i][j] == Square::from(p));
            col |= (0..self.dim).all(|j| self.board[j][i] == Square::from(p));
            diagl &= self.board[i][self.dim - 1 - i] == Square::from(p);
            diagr &= self.board[i][i] == Square::from(p);
        }

        row | col | diagl | diagr
    }

    pub fn is_draw(&self) -> bool {
        self.board.iter().flatten().all(|x| *x != Square::Empty)
    }

    pub fn negamax(&mut self, p: Player, mut alpha: i32, beta: i32) -> (i32, Option<Move>) {
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
                if self.board[r][c] != Square::Empty {
                    continue;
                }

                self.board[r][c] = Square::from(p);
                let (v, _) = self.negamax(p.opponent(), alpha, beta);
                self.board[r][c] = Square::Empty;

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

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Empty => write!(f, "."),
            Square::Player(p) => write!(f, "{}", p),
        }
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.dim {
            for col in 0..self.dim {
                write!(f, "{}", self.board[row][col])?
            }
            println!()
        }
        Ok(())
    }
}
