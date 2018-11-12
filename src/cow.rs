use nannou::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Cow {
    pub loc: usize,
    pub score: usize,
    next_move: Option<Move>,
}

impl Cow {
    pub fn new(loc: usize) -> Self {
        Self { loc: loc, score: 0, next_move: None }
    }

    pub fn compute_move(&mut self) {
        let mv = match random_range(0, 4) {
            0 => Move::UP,
            1 => Move::DOWN,
            2 => Move::LEFT,
            3 => Move::RIGHT,
            _ => panic!("rand error"),
        };

        self.next_move = Some(mv);
    }

    pub fn get_move(&self) -> Move {
        self.next_move.unwrap()
    }
}
