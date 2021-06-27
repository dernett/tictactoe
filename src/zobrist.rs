use crate::game::{Board, Player, Square};
use rand::Rng;

pub struct Zobrist {
    table: Vec<Vec<u64>>,
    dim: usize,
}

fn index_of(p: Player) -> usize {
    match p {
        Player::O => 0,
        Player::X => 1,
    }
}

impl Zobrist {
    pub fn new(dim: usize) -> Self {
        let mut rng = rand::thread_rng();
        Zobrist {
            dim,
            table: (0..dim * dim).map(|_| vec![rng.gen(), rng.gen()]).collect(),
        }
    }

    pub fn hash(&self, board: &Board) -> u64 {
        let mut h = 0;
        for i in 0..self.dim * self.dim {
            let (r, c) = (i / self.dim, i % self.dim);
            if let Square::Player(p) = board[r][c] {
                h ^= self.table[i][index_of(p)];
            }
        }
        h
    }
}
