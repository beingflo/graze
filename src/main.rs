extern crate nannou;
extern crate rand;

mod field;
mod cow;
mod evolution;
mod ui;
mod traits;

use nannou::prelude::*;
use nannou::event::SimpleWindowEvent;
use ui::UserInterface;
use evolution::Evolver;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    evolver: Evolver,
    ui: UserInterface,
}

fn model(app: &App) -> Model {
    let _ = app.new_window().with_title("Graze - Evolved").build().unwrap();

    let (width, height) = (720.0, 720.0);

    app.main_window().set_inner_size_points(width, height);

    let ui = app.new_ui().build().unwrap();
    let ui = UserInterface::new(ui);

    let mut evolver = Evolver::new(width, height, 50);
    evolver.field.init(10);

    evolver.evolve();

    Model { evolver: evolver, ui: ui }
}

fn event(_: &App, mut model: Model, event: Event) -> Model {
    match event {
        Event::Update(update) => {
            let dt = update.since_last.secs() as f32;

            model.evolver.step(dt);
            model.ui.update(dt);
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::Resized(size)), .. } => {
            model.evolver.field.update_size(size);
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::KeyPressed(nannou::VirtualKeyCode::Space)), .. } => {
            model.evolver.field.toggle_freeze();
        },

        _ => (),
    }
    model
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.evolver.field.draw(&draw);
    draw.to_frame(app, &frame).unwrap();

    frame
}
