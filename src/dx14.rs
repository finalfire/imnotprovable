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
    
    let transparent = rgba(1., 1., 1., 0.);
    let background = shex2dec("102542");
    let colors: Vec<Rgb> = [
        "f2f6d0","d0e1d4","d9d2b6","e4be9e","71697a",
        "3b3355","5d5d81","bfcde0","fefcfd",
        "a41623","f85e00","ffb563","ffd29d","918450"
    ].iter().map(|c| shex2dec(c)).collect();

    draw.background().color(background);
    
    let n = 13;
    let w = 41.;
    let h = (n as f32 * w) / 2.;
    let mut swap = false;


    let mut lines: Vec<Vec<Point2>> = Vec::new();

    // create points
    for i in 0..n {
        let mut line: Vec<Point2> = Vec::new();
        swap = !swap;

        for j in 0..n {

            let rx = random_range(-12.8, 12.8);
            let ry = random_range(-12.8, 12.8);

            let mut p = pt2(-h + (w * j as f32) + rx, -h + (w * i as f32) + ry);
            p.x += if swap { w / 2. } else { 0. };
            
            line.push(p);
        }

        lines.push(line.clone());
    }

    // draw triangles
    swap = true;
    for i in 0..lines.len() - 1 {

        let mut tri: Vec<Point2> = Vec::new();
        swap = !swap;
        
        for j in 0..lines[i].len() {
            tri.push(if swap { lines[i][j] } else { lines[i+1][j] });
            tri.push(if swap { lines[i+1][j] } else { lines[i][j] })
        }

        for j in 0..tri.len() - 2 {

            if random_f32() >= 0. {
                let c = pick_color(&colors);

                // fore
                draw.tri()
                    .color(rgba(c.red, c.green, c.blue, random_range(0.05, 0.75)))
                    .stroke_color(background)
                    .stroke_weight(random_range(0.65, 2.05))
                    .points(tri[j], tri[j+1], tri[j+2]);
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