use nannou::prelude::*;
use rand::Rng;

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
    pub id: usize,
}

impl Cow {
    pub fn new(loc: usize, id: usize) -> Self {
        Self { loc: loc, score: 0, next_move: None, id: id }
    }

    pub fn compute_move(&mut self, neighborhood: ([bool; 8], [bool; 8])) {
        let mut choices = vec![];

        if neighborhood.0[1] {
            choices.push(Move::UP);
        }
        if neighborhood.0[3] {
            choices.push(Move::LEFT);
        }
        if neighborhood.0[4] {
            choices.push(Move::RIGHT);
        }
        if neighborhood.0[6] {
            choices.push(Move::DOWN);
        }

        let mv = if choices.len() == 0 {
            match random_range(0, 4) {
                0 => Move::UP,
                1 => Move::DOWN,
                2 => Move::LEFT,
                3 => Move::RIGHT,
                _ => panic!("rand error"),
            }
        } else {
            *rand::thread_rng().choose(&choices).unwrap()
        };

        self.next_move = Some(mv);
    }

    pub fn get_move(&self) -> Move {
        self.next_move.unwrap()
    }
}
