

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

    draw.background().color(shex2dec("19323C"));

    // 
    let colors: Vec<Rgba> = [
        //"8aa399","7d84b2","8fa6cb","dbf4a7","d5f9de"
        //"f79256","fbd1a2","7dcfb6","00b2ca","1d4e89"
        "1f2041","4b3f72","ffc857","119da4","19647e"
    ].iter().map(|c| shex2deca(c)).collect();
    let pick_color = || { colors[random_range(0, colors.len())] };
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);

    let win_x = 768.0;
    let win_y = 768.0;

    let size = 4.75;

    let start_x = -(win_x / 2.);
    let start_y = win_y / 2.;

    let slope = 2.35;

    let max_w = 205;
    let max_h = 205;

    for i in 0..max_h {
        let color = pick_color();
        let alpha = random_range(0.10, 1.0) + 0.1;

        let offset_x = random_range(-0.10, 0.10);
        let offset_y = random_range(0., 0.10);

        if random_f32() < 0.25 {
            continue;
        }

        for j in 0..max_w {
            let py = start_x + (size * j as f32) + (offset_x * j as f32);
            let px = start_y - (size * i as f32) - (offset_y * i as f32);

            if random_f32() >= 0.002 {
                draw.ellipse()
                    .w_h(size, size)
                    .x_y(px, py)
                    .color(rgba(color.red, color.green, color.blue, alpha));
            }
        }
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