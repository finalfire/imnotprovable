mod dx;
use dx::{*};

use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    positions: Vec<Vector2>,
    color: Rgb,
}
impl Thing {
    pub fn new(p: Vector2, c: Rgb) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Thing { positions: positions, color: c }
    }
}

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
}

const N_THINGS: usize = 2000;

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .size(768,768)
        .view(view)
        .build()
        .unwrap();

    let colors: Vec<Rgb> = [
        "b80c09","0b4f6c","01baef","fbfbff","040f16",
        "655a7c","ab92bf","afc1d6","cef9f2","d6ca98"
    ].iter().map(|c| shex2dec(c)).collect();
    let pick_color = || { colors[random_range(0, colors.len())] };
    
    let mut things = Vec::new();
    for i in 0..N_THINGS {
        let thing = Thing::new(
            vec2((random::<f32>()-0.5) * 768.0, (random::<f32>()-0.5) * 768.0),
            pick_color()
        );
        things.push(thing);
    }

    let noise = Perlin::new();
    Model { things, noise }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 120.;
    let sn = 0.01 + time.cos() as f64 * 0.0005;
    for thing in model.things.iter_mut() {
        thing.positions.clear();
        thing.positions.push(vec2((random::<f32>()-0.5) * 768.0, (random::<f32>()-0.5) * 768.0));

        for k in 0..60 {
            let last = thing.positions[0];
            let new = last + vec2(
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32
            );
            thing.positions.insert(0, new);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.elapsed_frames() as f32 / 60.;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect().w_h(768., 768.).color(srgba(0.,0.,0.,0.1));

    for thing in model.things.iter() {
        draw.polyline().weight(1.11).points(thing.positions.iter().cloned()).color(thing.color);
        //draw.ellipse().xy(thing.position).radius(1.0).color(thing.color);
    }

    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    if app.elapsed_frames() == 1 || app.elapsed_frames() == 6 || app.elapsed_frames() == 12 || app.elapsed_frames() == 20 {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}