use cow::Move;

pub trait Mover {
    fn new(loc: usize, id: usize) -> Self;
    fn compute_move(&mut self, neighborhood: ([bool; 8], [bool; 8]));
    fn get_move(&self) -> Move;

    fn loc(&self) -> usize;
    fn set_loc(&mut self, loc: usize);
    fn score(&self) -> usize;
    fn inc_score(&mut self);
    fn reset_score(&mut self);
    fn id(&self) -> usize;
}
