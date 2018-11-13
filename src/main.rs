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
use field::Field;
use cow::Cow;


fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    field: Field<Cow>,
    ui: UserInterface,
}

fn model(app: &App) -> Model {
    let _ = app.new_window().with_title("Graze").build().unwrap();
    app.main_window().set_inner_size_points(720.0, 720.0);
    let (width, height) = app.main_window().inner_size_points();

    let ui = app.new_ui().build().unwrap();
    let ui = UserInterface::new(ui);

    let mut field = Field::new(width, height, 50);
    field.init(10);

    Model { field: field, ui: ui }
}

fn event(_: &App, mut model: Model, event: Event) -> Model {
    match event {
        Event::Update(update) => {
            let dt = update.since_last.secs() as f32;

            model.field.step(dt);
            model.ui.update(dt);
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::Resized(size)), .. } => {
            model.field.update_size(size);
        },

        Event::WindowEvent { simple: Some(SimpleWindowEvent::KeyPressed(nannou::VirtualKeyCode::Space)), .. } => {
            model.field.toggle_freeze();
        },

        _ => (),
    }
    model
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(WHITE);

    model.field.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
    model.ui.ui.draw_to_frame(app, &frame).unwrap();

    frame
}
