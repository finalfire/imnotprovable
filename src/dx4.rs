

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

    draw.background().color(shex2dec("D9E5D6"));
    let colors: Vec<Rgba> = [
        "E8DB7D","28C2FF","BB4430", "473198"//,"231F20"
    ].iter().map(|c| shex2deca(c)).collect();
    
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);
    let stroke_color = shex2dec("333333");

    let size = 32.;

    let start_x = -192.;
    let start_y = 192.;

    for i in 0..13 {
        for j in 0..13 {
            let pm_rot = { if random_f32() < 0.5 { -1 } else { 1 } };
            let pm_dis = { if random_f32() < 0.5 { -1 } else { 1 } };
            
            let ramt = j as f32 / size * PI / 180.0 * pm_rot as f32 * random_f32() * 26.85;
            let damt = j as f32 / size * pm_dis as f32 * random_f32() * 12.21;

            draw.rect()
                .x_y(start_x + (size * j as f32) + damt, start_y - (size * i as f32))
                .w_h(size, size)
                .rotate(ramt)
                .color({ if random_f32() <= 0.35 { colors[random_range(0, colors.len())] } else { transparent }})
                .stroke(stroke_color)
                .stroke_weight(1.75);

            if random_f32() < 0.25 {
                draw.rect()
                    .x_y(start_x + (size * j as f32) + damt, start_y - (size * i as f32))
                    .w_h(size, size)
                    .rotate(ramt + random_range(-0.25, 0.25))
                    .color(transparent)
                    .stroke(rgba(0.0, 0.0, 0.0, random_range(0.45, 0.85)))
                    .stroke_weight(0.98);
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