use rand::Rng;

use cow::Move;
use field::Field;
use traits::Mover;

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

impl Mover for MutantCow {
    fn new(loc: usize, id: usize) -> Self {
        Self { loc: loc, score: 0, next_move: None, id: id }
    }

    fn compute_move(&mut self, neighborhood: ([bool; 8], [bool; 8])) {
    }

    fn get_move(&self) -> Move {
        self.next_move.unwrap()
    }

    fn loc(&self) -> usize {
        self.loc
    }

    fn set_loc(&mut self, loc: usize) {
        self.loc = loc;
    }

    fn score(&self) -> usize {
        self.score
    }

    fn inc_score(&mut self) {
        self.score += 1;
    }

    fn id(&self) -> usize {
        self.id
    }
}
