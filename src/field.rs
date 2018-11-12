use nannou::prelude::*;
use nannou::draw::Draw;

use cow::Cow;

pub struct Field {
    cows: Vec<Cow>,
    patches: Vec<u32>,

    size: usize,
    width: f32,
    height: f32,

    freeze: bool,
}

impl Field {
    pub fn new(width: f32, height: f32, size: usize) -> Self {
        Self { cows: Vec::new(), patches: vec![0; size*size], size: size, width: width, height: height, freeze: false }
    }

    pub fn toggle_freeze(&mut self) {
        self.freeze = !self.freeze;
    }

    pub fn update_size(&mut self, size: Vector2) {
        self.width = size.x;
        self.height = size.y;
    }

    pub fn draw(&self, draw: &Draw) {
        for (idx, patch) in self.patches.iter().enumerate() {
            let mut color = RED;
            if *patch == 0 {
                // Recovered
                color = GREEN;
            }

            let (x,y) = (idx % self.size, idx / self.size);
            let x = (x as f32 / self.size as f32) * self.width - 0.5 * self.width;
            let y = (y as f32 / self.size as f32) * self.height - 0.5 * self.height;
            draw.rect().w_h(20.0, 20.0).x_y(x, y).color(color);
            println!("{} {}", x, y);
        }
    }

    pub fn step(&mut self, dt: f32) {
    }

}
