

mod dx;
use dx::{*};

use nannou::prelude::*;

fn view(app: &App, frame: Frame) {
    // do not write after the first frame
    if app.elapsed_frames() > 1 {
        return;
    }

    // Begin drawing
    let draw = app.draw();

    let colors: Vec<Rgb> = [
        "0fa3b1","d9e5d6","eddea4","f7a072","ff9b42",
        "7d4e57", "d66853",
        "4357ad","48a9a6","e4dfda","d4b483","c1666b"
    ].iter().map(|c| shex2dec(c)).collect();
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);

    draw.background().color(shex2dec("f3f5eb"));

    let start_x = -194.;
    let start_y = 188.;

    let size = 33.;
    let offset = 31.0;

    for k in 0..5 {
        for i in 0..8 {
            for j in 0..8 {

                let dmt = (i as f32 / size) * random_f32() * 50.;
                println!("{}, {}, {}", i, size, dmt);

                let px = start_x + (size * j as f32) + (offset * j as f32);// + dmt;
                let py = start_y - (size * i as f32) - (offset * i as f32);

                let rnd_color = pick_color(&colors);
                let alpha = random_range(0.5, 1.0);

                let curr_angle = if true { 0. } else { random_range(-90., 90.) };
                let curr_size = if k == 0 { size } else { size * (1. / k as f32) };

                let curr_weight = if true { random_range(0.90, 0.99) } else { random_range(0.25, 0.85) };

                draw.rect()
                    .x_y(px, py)
                    .w_h(curr_size, curr_size)
                    .color(transparent)
                    .z_degrees(curr_angle)
                    .stroke_color(rgba(rnd_color.red, rnd_color.green, rnd_color.blue, alpha))
                    .stroke_weight(curr_weight);
            }
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    if app.elapsed_frames() == 1 {
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

fn main() {
    nannou::sketch(view).size(768,768).run();
}