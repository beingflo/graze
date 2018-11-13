use nannou::prelude::*;
use nannou::draw::Draw;

use rand::{thread_rng, Rng};

use cow::Move;
use traits::Mover;

pub struct Field<T: Mover> {
    cows: Vec<T>,
    patches: Vec<u32>,

    size: usize,
    width: f32,
    height: f32,

    freeze: bool,
    last_step: f32,
    best: usize,
    step: usize,

    id_gen: usize,
}

impl<T: Mover> Field<T> {
    pub fn new(width: f32, height: f32, size: usize) -> Self {
        Self { cows: Vec::new(), patches: vec![0; size*size], size: size, width: width, height: height, freeze: false, last_step: 0.0, step: 0, best: 0, id_gen: 0 }
    }

    pub fn init(&mut self, n: usize) {
        for _ in 0..n {
            let loc = random_range(0, self.size*self.size);
            self.add_cow(loc);
        }
    }

    pub fn toggle_freeze(&mut self) {
        self.freeze = !self.freeze;
    }

    pub fn update_size(&mut self, size: Vector2) {
        self.width = size.x;
        self.height = size.y;
    }

    pub fn add_cow(&mut self, loc: usize) {
        self.cows.push(T::new(loc, self.id_gen));
        self.id_gen += 1;
    }

    pub fn statistics(&mut self) -> (usize, usize, f32) {
        let mut sum = 0;
        let mut best = 0;
        let mut worst = self.cows[0].score();

        for c in self.cows.iter() {
            if c.score() > best {
                best = c.score();
                self.best = c.id();
            }
            if c.score() < worst {
                worst = c.score();
            }

            sum += c.score();
        }

        let av = sum as f32 / self.cows.len() as f32;

        (best, worst, av)
    }

    pub fn draw(&self, draw: &Draw) {
        for (idx, patch) in self.patches.iter().enumerate() {
            let mut color = RED;
            if *patch == 0 {
                // Recovered
                color = DARK_GREEN;
            }

            let (x,y) = (idx % self.size, idx / self.size);
            let x = ((x as f32 + 0.5) / self.size as f32) * self.width - 0.5 * self.width;
            let y = ((y as f32 + 0.5) / self.size as f32) * self.height - 0.5 * self.height;

            let w = (self.width / self.size as f32) * 0.9;
            let h = (self.height / self.size as f32) * 0.9;

            draw.rect().w_h(w, h).x_y(x, y).color(color);
        }

        for c in self.cows.iter() {
            let radius = 5.0;
            let (x,y) = (c.loc() % self.size, c.loc() / self.size);

            let x = ((x as f32 + 0.5) / self.size as f32) * self.width - 0.5 * self.width;
            let y = ((y as f32 + 0.5) / self.size as f32) * self.height - 0.5 * self.height;

            let mut color = BLACK;
            if c.id() == self.best {
                color = WHITE;
            }

            draw.ellipse().radius(radius).x_y(x, y).color(color);
        }
    }

    fn move_cows(&mut self) {
        let mut cow_pos = vec![0; self.size*self.size];
        for c in &self.cows {
            cow_pos[c.loc()] += 1;
        }

        for c in &mut self.cows {
            let neigh = get_neighborhood(c.loc(), self.size);
            // Borrow here because closure tries to borrow all of self otherwise
            let p = &self.patches;
            let patches_vec: Vec<bool> = neigh.iter().map(|&idx| p[idx] == 0).collect();
            let mut patches = [false; 8];
            patches.copy_from_slice(&patches_vec[0..8]);

            let cows_vec: Vec<bool> = neigh.iter().map(|&idx| cow_pos[idx] > 0).collect();
            let mut cows = [false; 8];
            cows.copy_from_slice(&cows_vec[0..8]);

            c.compute_move((patches, cows));
        }

        for c in &mut self.cows {
            move_cow(c, self.size);
        }
    }

    fn eat(&mut self) {
        let grass_regen = 50;
        thread_rng().shuffle(&mut self.cows);

        for c in self.cows.iter_mut() {
            if self.patches[c.loc()] == 0 {
                c.inc_score();
                self.patches[c.loc()] = grass_regen;
            }
        }
    }

    fn recover_grass(&mut self) {
        for p in self.patches.iter_mut() {
            if *p > 0 {
                *p -= 1;
            }
        }
    }

    pub fn print_statistics(&mut self) {
        let (best, worst, av) = self.statistics();

        println!("Step: {}\nBest score: {}\nAverage score: {}\nWorst score: {}\n", self.step, best, av, worst);
    }

    pub fn step(&mut self, dt: f32) {
        let step_frequency = 10;
        let step_interval = 1.0 / step_frequency as f32;

        self.last_step += dt;

        if self.last_step < step_interval {
            return;
        }
        self.last_step = 0.0;

        if self.freeze {
            return;
        }

        self.step += 1;

        self.move_cows();
        self.eat();
        self.recover_grass();
    }

    pub fn cows(&mut self) -> &mut Vec<T> {
        &mut self.cows
    }

    pub fn reset(&mut self) {
        for c in &mut self.cows {
            c.reset_score();
            let loc = random_range(0, self.size*self.size);
            c.set_loc(loc);
        }
        self.step = 0;

        for p in self.patches.iter_mut() {
            *p = 0;
        }
    }
}

fn move_cow<T: Mover>(cow: &mut T, size: usize) {
    let (mut x, mut y) = (cow.loc() % size, cow.loc() / size);

    match cow.get_move() {
        Move::UP => if y == size - 1 { y = 0 } else { y += 1 },
        Move::DOWN => if y == 0 { y = size - 1 } else { y -= 1 },
        Move::LEFT => if x == 0 { x = size - 1 } else { x -= 1 },
        Move::RIGHT => if x == size - 1 { x = 0 } else { x += 1 },
    }

    cow.set_loc(y * size + x);
}

fn get_neighborhood(loc: usize, size: usize) -> [usize; 8] {
    let (x, y) = (loc % size, loc / size);

    let mut neigh = [0; 8];

    let x0 = if x == 0 { size - 1 } else { x - 1 };
    let x1 = x;
    let x2 = if x == size - 1 { 0 } else { x + 1 };

    let y0 = if y == size - 1 { 0 } else { y + 1 };
    let y1 = y;
    let y2 = if y == 0 { size - 1 } else { y - 1 };

    neigh[0] = y0 * size + x0;
    neigh[1] = y0 * size + x1;
    neigh[2] = y0 * size + x2;
    neigh[3] = y1 * size + x0;
    neigh[4] = y1 * size + x2;
    neigh[5] = y2 * size + x0;
    neigh[6] = y2 * size + x1;
    neigh[7] = y2 * size + x2;

    neigh
}
