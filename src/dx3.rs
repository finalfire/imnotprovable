

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

    draw.background().color(shex2dec("000000"));

    let colors: Vec<Rgb> = [
        "26547c","ef476f","ffd166","06d6a0","fffcf9",
        "44355b","31263e","221e22","eca72c","ee5622",
        "e4fde1","8acb88","648381","575761","ffbf46",
        "241623","d0cd94","3c787e","c7ef00","d56f3e"
    ].iter().map(|c| shex2dec(c)).collect();

    let win = app.window_rect();
    let win_p = win.pad(89.0);

    /*draw.rect()
        .xy(win_p.xy())
        .wh(win_p.wh())
        .color(rgba(0.3, 0.4, 0.7, 0.1));*/
    
    let sx = 23.0;
    let sy = 23.0;
    let w = 21.0;
    let row = 34;
    let col = 21;

    let offset_x = 265.0;
    let offset_y = 395.0;

    for i in 0..row {
        for j in 0..col {
            let ox = (j+1) as f32;
            let oy = (i+1) as f32;
            
            let rnd = random_f32();

            if rnd >= 0.35 && rnd <= 0.85 {
                // draw triangle
                let p: Vec<Point2> = [
                    pt2(sx * ox, sy * oy),
                    pt2((sx * ox) + w, sy * oy),
                    pt2(sx * ox, (sy * oy) - w)
                ].iter().map(|p| pt2(p.x - offset_x, p.y - offset_y)).collect();

                draw.tri()
                    .color(colors[random_range(0, colors.len())])
                    .points(p[0], p[1], p[2]);
            } else if rnd > 0.85 {
                // draw square
                let c = colors[random_range(0, colors.len())];
                let p: Vec<Point2> = [
                    pt2(sx * ox, sy * oy),
                    pt2((sx * ox) + w, sy * oy),
                    pt2(sx * ox, (sy * oy) - w),
                    pt2((sx * ox) + w, (sy * oy) - w)
                ].iter().map(|p| pt2(p.x - offset_x, p.y - offset_y)).collect();
                draw.quad()
                    .color(rgba(c.red, c.green, c.blue, random_range(0.55, 1.0)))
                    .points(p[0], p[1], p[3], p[2]);

            } else {
                if random_f32() >= 0.90 {
                    let p: Vec<Point2> = [
                    pt2(sx * ox, sy * oy),
                    pt2((sx * ox) + w, sy * oy),
                    pt2(sx * ox, (sy * oy) - w),
                    pt2((sx * ox) + w, (sy * oy) - w)
                ].iter().map(|p| pt2(p.x - offset_x, p.y - offset_y)).collect();
                draw.quad()
                    .color(rgba8(211, 211, 211, 128))
                    .points(p[0], p[1], p[3], p[2]);
                }
            }

            if random_f32() >= 0.85 {
                let rnd_color = colors[random_range(0, colors.len())];
                let rnd_w = w * random_range(-1.25, 1.25);

                let p: Vec<Point2> = [
                    pt2(sx * ox, sy * oy),
                    pt2((sx * ox) + w + rnd_w, sy * oy),
                    pt2(sx * ox, (sy * oy) - w - rnd_w),
                    pt2((sx * ox) + w + rnd_w, (sy * oy) - w - rnd_w)
                ].iter().map(|p| pt2(p.x - offset_x, p.y - offset_y)).collect();
                draw.quad()
                    .color(rgba(rnd_color.red, rnd_color.green, rnd_color.blue, random_range(0.15, 0.35)))
                    .stroke_weight(random_range(0.05, 0.25))
                    .points(p[0], p[1], p[3], p[2]);
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