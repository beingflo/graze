use rand::Rng;

use cow::Move;
use field::Field;

struct Evolver {
}

impl Evolver {

}

struct MutantCow {
    pub loc: usize,
    pub score: usize,
    next_move: Option<Move>,
    pub id: usize,
}

impl MutantCow {
    pub fn new(loc: usize, id: usize) -> Self {
        Self { loc: loc, score: 0, next_move: None, id: id }
    }

    pub fn compute_move(&mut self, neighborhood: ([bool; 8], [bool; 8])) {
    }

    pub fn get_move(&self) -> Move {
        self.next_move.unwrap()
    }
}
