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

    let n = 33;
    let m = 11;
    let w = 417.0;

    let step_x = w / m as f32;
    let step_y = 10.0;

    let half_point_x = (w - step_x) / 2.0;
    let half_point_y = ((n - 1) as f32 * step_y) / 2.0;

    type Point2VV = Vec<Vec<Point2>>;
    let mut lines: Point2VV = Vec::new();

    let background = hex2deca("ed", "b8", "8b", 1.0);
    let foreground = hex2deca("2e", "28", "2a", 1.0);

    draw.background().color(background);

    for i in 0..n {
        let mut points: Vec<Point2<f32>> = Vec::new();

        for p in 0..m {
            let x = (p as f32 * step_x) - half_point_x;
            let mut y = (i as f32 * step_y) - half_point_y;

            if i < n / 2 && p > m / 2 {
                if random_f32() >= 0.25 {
                    y += random_range(-6.5, 7.5);                
                } else {
                    y += random_range(-11.23, 14.23);                
                }
            } else {
                if random_f32() >= 0.65 {
                    y += random_range(-1.89, 1.89);
                } 
            }

            points.push(pt2(x,y));
        }

        draw.polyline()
            .weight(1.0)
            .points_colored(points.iter().map(|p| (*p, foreground)));

        for k in 0..5 {
            let shadows = points.iter()
                .map(|p| {
                    let y = p.y + random_range(-1.23, 1.23);
                    pt2(p.x, y)
                });
            
            draw.polyline()
                .weight(0.8)
                .points_colored(shadows.map(|p| { 
                    (*p, rgba(foreground.red, foreground.green, foreground.blue, random_range(0.05, 0.45))
                )}));
        }
        lines.push(points);
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