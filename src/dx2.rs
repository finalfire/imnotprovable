

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

    draw.background().color(WHITE);

    let colors: Vec<Rgb> = [
        //"160f29","246a73","368f8b","f3dfc1","ddbea8",
        "d3f8e2","e4c1f9","f694c1","ede7b1","a9def9",
        "2e86ab","a23b72","f18f01","c73e1d","3b1f2b",
        "7a306c","8e8dbe","a9e4ef","81f495","96f550"
    ].iter().map(|c| shex2dec(c)).collect();

    let win = app.window_rect();
    let win_p = win.pad(89.0);

    let mut els: Vec<Rect> = Vec::new();

    let r = Rect::from_w_h(34.0, 34.0).top_left_of(win_p);
    draw.rect()
        .xy(r.xy())
        .wh(r.wh())
        .color(colors[random_range(0, colors.len())]);
    
    els.push(r);

    let rows = 25;
    let columns = 25;

    for i in 0..rows {
        for j in 0..columns {
            if i == 0 && j == 0 { continue; }
            let last = {
                if j == 0 { els.get((i - 1) * columns).expect(":(") }
                else { els.last().expect(":(") }
            };

            let curr = {
                if j == 0 { last.below(*last).shift_y(-3.0) }
                else { last.right_of(*last).shift_x(3.0) }
            };

            draw.rect()
                .x_y(curr.x() + random_range(-3.85, 3.85), curr.y() + random_range(-3.85, 3.85))
                .wh(curr.wh())
                .color(colors[random_range(0, colors.len())]);

            for k in 0..3 {
                let dim = random_range(8.0, 89.0);
                let c = colors[random_range(0, colors.len())];
                draw.rect()
                    .x_y(curr.x() + random_range(-4.85, 4.85), curr.y() + random_range(-4.85, 4.85))
                    .w_h(dim, dim)
                    .z_degrees({
                        if random_f32() >= 0.75 { random_range(-180.0, 180.0) } else { 0.0 }
                    })
                    .color(rgba(c.red, c.green, c.blue, random_range(0.05, 0.25)));
            }
            
            els.push(curr);
        }
    }

    /*for j in 1..25 {
        let last = els.last().expect("No element");
        let curr = last.right_of(*last).shift_x(3.0);
        draw.rect()
            .xy(curr.xy())
            .wh(curr.wh())
            .color(SALMON);
        els.push(curr);
    }*/

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