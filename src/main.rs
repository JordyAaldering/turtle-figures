mod seq;

use seq::*;

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Model {
    seq: Seq<i32>,
    points: Vec<Point2>,
    settings: Settings,
    egui: Egui,
}

struct Settings {
    angle0: f32,
    angle1: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    let mut seq = Seq::new(0, doubling);
    let points = update_points(&mut seq, 140.0, 280.0);

    Model {
        seq,
        points,
        egui,
        settings: Settings {
            angle0: 140.0,
            angle1: 280.0,
        },
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();
    let changed = egui::Window::new("Settings")
        .show(&ctx, |ui| {
            ui.label("Angle 0:");
            let changed0 = ui
                .add(egui::Slider::new(&mut settings.angle0, 0.0..=360.0))
                .changed();

            ui.label("Angle 1:");
            let changed1 = ui
                .add(egui::Slider::new(&mut settings.angle1, 0.0..=360.0))
                .changed();

            changed0 || changed1
        })
        .unwrap();

    if changed.inner.unwrap() {
        model.points = update_points(&mut model.seq, settings.angle0, settings.angle1);
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let mut from = pt2(0.0, 0.0);
    for to in model.points.iter() {
        draw.line().start(from).end(*to).color(STEELBLUE);
        from = *to;
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn update_points(seq: &mut Seq<i32>, angle0: f32, angle1: f32) -> Vec<Point2> {
    let radians = seq_to_radians(seq, angle0, angle1);
    let points = rad_to_points(radians);
    points.collect()
}

fn seq_to_radians(seq: &mut Seq<i32>, angle0: f32, angle1: f32) -> impl Iterator<Item = f32> + '_ {
    const DEG2RAD: f32 = (PI * 2.0) / 360.0;
    seq.take(9216).iter().scan(0.0, move |angle, &x| {
        *angle += if x == 0 { angle0 } else { angle1 };
        Some(*angle * DEG2RAD)
    })
}

fn rad_to_points<'a>(rad: impl Iterator<Item = f32> + 'a) -> impl Iterator<Item = Point2> + 'a {
    rad.scan(pt2(0.0, 0.0), |from, rad| {
        *from += pt2(rad.cos(), rad.sin()) * 20.0;
        Some(*from)
    })
}
