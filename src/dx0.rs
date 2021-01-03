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

    // Background
    draw.background().color(rgb8(57, 147, 221));

    let n = 13;
    let m = 10;
    let w = 421.0;
    let weight = 1.85;

    let step_x = w / m as f32;
    let step_y = 24.0;

    let half_point_x = (w - step_x) / 2.0;
    let half_point_y = ((n - 1) as f32 * step_y) / 2.0;

    for i in 0..n {
        let points: Vec<Point2<f32>> = (0..m)
            .map(|p| {
                let x = (p as f32 * step_x) - half_point_x;
                let mut y = (i as f32 * step_y) - half_point_y;
                y = y + random_range(-11.8, 13.8);
                pt2(x, y)
            })
            .collect();

        draw.polyline()
            .weight(weight)
            .points_colored(points.iter().map(|p| (*p, WHITE)));
        
        for j in 0..3 {
            let shadows = points.iter()
                .map(|p| {
                    let x = p.x;
                    let y = p.y + random_range(-4.0 * (j+1) as f32, 4.0 * (j+1) as f32);
                    (pt2(x, y), rgba(1.0, 1.0, 1.0, 0.25))
                });

            draw.polyline()
                .weight(weight * 0.25)
                .points_colored(shadows);
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