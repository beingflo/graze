use nannou::prelude::*;
use nannou::draw::Draw;

use cow::Cow;
use cow::Move;

pub struct Field {
    cows: Vec<Cow>,
    patches: Vec<u32>,

    size: usize,
    width: f32,
    height: f32,

    freeze: bool,
    last_step: f32,
}

impl Field {
    pub fn new(width: f32, height: f32, size: usize) -> Self {
        Self { cows: Vec::new(), patches: vec![0; size*size], size: size, width: width, height: height, freeze: false, last_step: 0.0 }
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
        self.cows.push(Cow::new(loc));
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
            let (x,y) = (c.loc % self.size, c.loc / self.size);

            let x = ((x as f32 + 0.5) / self.size as f32) * self.width - 0.5 * self.width;
            let y = ((y as f32 + 0.5) / self.size as f32) * self.height - 0.5 * self.height;

            draw.ellipse().radius(radius).x_y(x, y).color(BLACK);
        }
    }

    pub fn step(&mut self, dt: f32) {
        let step_frequency = 2;
        let step_interval = 1.0 / step_frequency as f32;
        let grass_regen = 20;

        self.last_step += dt;

        if self.last_step < step_interval {
            return;
        }
        self.last_step = 0.0;

        // Move cows
        for c in &mut self.cows {
            c.compute_move();
        }

        for c in &mut self.cows {
            move_cow(c, self.size);
        }

        // Eat grass
        for c in &mut self.cows {
            if self.patches[c.loc] == 0 {
                c.score += 1;
                self.patches[c.loc] = grass_regen;
            }
        }

        // Recover grass
        for p in self.patches.iter_mut() {
            if *p > 0 {
                *p -= 1;
            }
        }
    }

}

fn move_cow(cow: &mut Cow, size: usize) {
    let (mut x, mut y) = (cow.loc % size, cow.loc / size);

    match cow.get_move() {
        Move::UP => if y == size - 1 { y = 0 } else { y += 1 },
        Move::DOWN => if y == 0 { y = size - 1 } else { y -= 1 },
        Move::LEFT => if x == 0 { x = size - 1 } else { x -= 1 },
        Move::RIGHT => if x == size - 1 { x = 0 } else { x += 1 },
    }

    cow.loc = y * size + x;
}
