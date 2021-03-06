

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
        "6c9a8b","e8998d","eed2cc","fbf7f4","a1683a",
        "f18f01","048ba8","2e4057","99c24d"
    ].iter().map(|c| shex2deca(c)).collect();
    
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);

    let win_x = 824.0;
    let win_y = 824.0;

    let size = 21.;

    let offset = 0.;
    let start_x = -(win_x / 2.) - (size / 2.) + 2.5;
    let start_y = win_y / 2. + (size / 2.) - 2.5;

    let mut positions: Vec<(usize, usize)> = Vec::new();
    for i in 0..41 {
        for j in 0..41 {
            positions.push((i,j));
        }
    }

    let mut still_active: Vec<usize> = (0..positions.len()).collect();
    
    let mut next_element = random_range(0, still_active.len());
    loop {
        let (i,j) = positions[still_active[next_element]];
        still_active.remove(next_element as usize);

        let px = start_x + (size * j as f32) + (offset * j as f32);
        let py = start_y - (size * i as f32) - (offset * i as f32);

        let stroke_color = colors[random_range(0, colors.len())];
        let stroke_weight = random_range(0.1, 6.35);

        if random_f32() > 0.05 {
            let curr_size = {
                if random_f32() < 0.25 {
                    size - (size * random_range(0.15, 0.65))
                } else {
                    size
                }
            };

            let mut curr_px = px;
            let mut curr_py = py;

            if random_f32() < 0.15 {
                curr_px += random_range(-5., 5.);
                curr_py += random_range(-5., 5.);
            }

            if random_f32() >= 0.05 {
                draw.ellipse()
                    .x_y(curr_px, curr_py)
                    .w_h(curr_size, curr_size)
                    .color(transparent)
                    .stroke(stroke_color)
                    .stroke_weight(stroke_weight);
            } else {
                draw.ellipse()
                    .x_y(curr_px, curr_py)
                    .w_h(curr_size, curr_size)
                    .color(stroke_color)
                    .stroke(shex2dec("333333"))
                    .stroke_weight(stroke_weight);
            }
        }
        
        if still_active.len() > 0 {
            next_element = random_range(0, still_active.len());
        } else {
            break;
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