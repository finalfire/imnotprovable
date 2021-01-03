

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

    let colors: Vec<Rgba> = [
        //"8aa399","7d84b2","8fa6cb","dbf4a7","d5f9de"
        "f79256","fbd1a2","7dcfb6","00b2ca","1d4e89"
    ].iter().map(|c| shex2deca(c)).collect();
    let pick_color = || { colors[random_range(0, colors.len())] };
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);

    let win_x = 768.0;
    let win_y = 768.0;

    let size = 32.75;

    let start_x = -(win_x / 2.);
    let start_y = win_y / 2.;

    let max_w = 205;
    let max_h = 205;

    for i in 0..max_h {
        let color = pick_color();
        let alpha = random_range(0.10, 1.0) + 0.1;

        let offset_x = random_range(-0.10, 0.10);
        let offset_y = random_range(0.10, 0.25);

        let span = (offset_x * max_w as f32) / 2.;

        for j in 0..max_w {

            let noise_x = random_range(-0.55, 0.35);
            let noise_y = {
                if random_f32() >= 0.0025 {
                    random_range(-0.15, 0.15)
                } else {
                    random_range(-5.15, 5.15)
                }
            };

            let px = start_x + (size * j as f32) + (offset_x * j as f32) - span + noise_x;
            let py = start_y - (size * i as f32) - (offset_y * i as f32) + noise_y;

            if random_f32() >= 0.001 {
                if random_f32() >= 0.05 {
                    draw.ellipse()
                        .w_h(size, size)
                        .x_y(px, py)
                        .color(rgba(color.red, color.green, color.blue, alpha));
                } else {
                    draw.ellipse()
                        .w_h(size*2., size*2.)
                        .x_y(px, py)
                        .color(transparent)
                        .stroke(WHITE)
                        .stroke_weight(1.25);
                }
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