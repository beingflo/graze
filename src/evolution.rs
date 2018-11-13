use nannou::prelude::*;

use cow::Move;
use field::Field;
use traits::Mover;

pub struct Evolver {
    pub field: Field<MutantCow>,
}

impl Evolver {
    pub fn new(width: f32, height: f32, size: usize) -> Self {
        let field = Field::new(width, height, size);
        Self { field: field }
    }

    pub fn evolve(&mut self) {
        let g = 100;
        let n = 100;

        for _ in 0..g {
            self.select(n);
            self.mutate();

            self.field.print_statistics();

            self.field.reset();
        }
    }

    fn select(&mut self, n: usize) {
        for _ in 0..n {
            self.step(1.0);
        }

        let cows = self.field.cows();

        cows.sort_by(|a, b| b.score.cmp(&a.score));

        let mid = cows.len() / 2;

        for i in mid..cows.len() {
            let a = random_range(0, mid);
            let b = random_range(0, mid);

            for x in 0..4 {
                cows[i].move_prob[x] = (cows[a].move_prob[x] + cows[b].move_prob[x]) / 2.0;
            }
        }
    }

    fn mutate(&mut self) {
    }

    pub fn step(&mut self, dt: f32) {
        self.field.step(dt);
    }

}

pub struct MutantCow {
    pub loc: usize,
    pub score: usize,
    next_move: Option<Move>,
    pub id: usize,

    move_prob: [f32; 4],
}

impl Mover for MutantCow {
    fn new(loc: usize, id: usize) -> Self {
        let mut move_prob = [0.0; 4];
        let mut cum = 0.0;
        for i in 0..3 {
            let r = random_f32(); 
            let r = r * (1.0 - cum);

            cum += r;

            move_prob[i] = r;
        }
        move_prob[3] = 1.0 - cum;

        Self { loc: loc, score: 0, next_move: None, id: id, move_prob: move_prob }
    }

    fn compute_move(&mut self, _neighborhood: ([bool; 8], [bool; 8])) {
        let r = random_f32();

        let mut cum = 0.0;
        let mut choice = 0;
        for (i, p) in self.move_prob.iter().enumerate() {
            cum += p;
            if r < cum {
                choice = i;
                break;
            }
        }

        let mv = match choice {
            0 => Move::UP,
            1 => Move::DOWN,
            2 => Move::LEFT,
            3 => Move::RIGHT,
            _ => panic!("rand error"),
        };

        self.next_move = Some(mv);
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

    fn reset_score(&mut self) {
        self.score = 0;
    }

    fn id(&self) -> usize {
        self.id
    }
}
