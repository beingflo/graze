pub struct Cow {
    pub loc: (usize, usize),
}

impl Cow {
    pub fn new(x: usize, y: usize) -> Self {
        Self { loc: (x, y) }
    }
}
