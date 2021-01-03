

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

    draw.background().color(shex2dec("fefefe"));
    let colors: Vec<Rgba> = [
        "8eb1c7","b02e0c","eb4511","c1bfb5","fefdff",
        "595f72","575d90","84a07c","c3d350","e6f14a",
        "541388","d90368","f1e9da","2e294e","ffd400"
    ].iter().map(|c| shex2deca(c)).collect();
    
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);
    let stroke_color = shex2dec("333333");

    let size = 32.;

    let start_x = -224.;
    let start_y = 208.;

    let broken = {
        if random_f32() <= 0.25 {
            random_range(0, 15)
        } else { -1 }
    };

    println!("{}", broken);

    for i in 0..15 {

        if i == broken { continue; }

        for j in 0..15 {

            if j == broken { continue; }

            let pm_rot = { if random_f32() < 0.5 { -1 } else { 1 } };
            let pm_dis = { if random_f32() < 0.5 { -1 } else { 1 } };
            
            let ramt = (14-i) as f32 / size * PI / 180.0 * pm_rot as f32 * random_f32() * 28.85;
            let damt = (14-i) as f32 / size * pm_dis as f32 * random_f32() * 14.71;

            let px = start_x + (size * j as f32) + damt;
            let py = start_y - (size * i as f32);

            if random_f32() <= 0.95 {
                // shadow
                if random_f32() <= 0.75 {
                    draw.rect()
                        .x_y(px + 5., py - 5.)
                        .w_h(size, size)
                        .rotate(ramt)
                        .color(rgba(0., 0., 0., random_range(0.10, 0.25)));
                }

                // original
                draw.rect()
                    .x_y(px, py)
                    .w_h(size, size)
                    .rotate(ramt)
                    .color({ if random_f32() <= 0.35 { colors[random_range(0, colors.len())] } else { transparent }})
                    .stroke(stroke_color)
                    .stroke_weight(random_range(0.85, 1.76));

                if random_f32() < 0.25 {
                    draw.rect()
                        .x_y(px, py)
                        .w_h(size, size)
                        .rotate(ramt + random_range(-0.25, 0.25))
                        .color(transparent)
                        .stroke(rgba(0.0, 0.0, 0.0, random_range(0.45, 0.85)))
                        .stroke_weight(random_range(0.33, 1.35));
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