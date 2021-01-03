mod dx;
use dx::{*};

use nannou::prelude::*;

struct Circle {
    xy: Point2,
    r: f32
}

impl Circle {
    fn new(xy: Point2, r: f32) -> Circle {
        Circle {xy, r}
    }
}

fn collision(circle: &Circle, circles: &Vec<Circle>, boundary: f32) -> bool {
    let d = circle.r / 2.;
    for c in circles.iter() {
        let a = d + (c.r/2.);
        let x = circle.xy.x - c.xy.x;
        let y = circle.xy.y - c.xy.y;

        if a >= ((x*x) + (y*y)).sqrt() * 1.15 {
            return true
        }
    }

    if circle.xy.x + circle.r >= boundary || circle.xy.x - circle.r <= -boundary {
        return true
    }

    if circle.xy.y + circle.r >= boundary || circle.xy.y - circle.r <= -boundary {
        return true
    }

    false
}

fn view(app: &App, frame: Frame) {
    // do not write after the first frame
    if app.elapsed_frames() > 1 {
        return;
    }

    // Begin drawing
    let draw = app.draw();

    let colors: Vec<Rgb> = [
        "75dbcd","c9dbba","dcdba8","f5cda7","faa381",
        "fbba72","ca5310","bb4d00","8f250c","691e06",
        "06aed5","086788","f0c808","fff1d0","dd1c1a",
        "a8e0ff","8ee3f5","70cad1","3e517a","b08ea2",
        "ed6a5a","f4f1bb","9bc1bc","e6ebe0","36c9c6"

    ].iter().map(|c| shex2dec(c)).collect();
    let transparent = rgba(1., 1., 1., 0.);

    draw.background().color(shex2dec("F7F3E3"));

    let n = 5520;
    let r_min = 2.0;
    let r_max = 210.0;
    let half_window = 768. / 2.;
    let tries = 1000;

    let boundary = half_window;

    println!("boundary: {}", boundary);

    /*draw.rect()
        .w_h(-2.*boundary, boundary*2.)
        .color(transparent)
        .stroke_weight(1.0)
        .stroke_color(WHITE);*/

    let mut circles: Vec<Circle> = Vec::new();
    for _i in 0..n {
        let mut safe = false;
        let mut circle: Circle = Circle::new(pt2(random_range(-boundary, boundary), random_range(-boundary, boundary)), r_min);

        for _try in 0..tries {
            if !collision(&circle, &circles, boundary) {
                safe = true;
                break;
            }
            circle = Circle::new(pt2(random_range(-boundary, boundary), random_range(-boundary, boundary)), r_min);
        }

        if !safe {
            continue;
        }

        for r in (r_min as usize + 1)..(r_max as usize) {
            circle.r = r as f32;
            if collision(&circle, &circles, boundary) { // non basta fare -1.0
                circle.r -= 1.0;
                break;
            }
        }

        /*let color = pick_color(&colors);
        let alpha = 1.;
        if true { //circle.r < r_max * 0.25 {
            draw.ellipse()
                .xy(circle.xy)
                .w_h(circle.r, circle.r)
                .color(transparent)
                .stroke_weight(0.9)
                .stroke_color(rgba(color.red, color.green, color.blue, alpha));
        } else {
            draw.ellipse()
                .xy(circle.xy)
                .w_h(circle.r, circle.r)
                .color(color);
        }*/

        let color = pick_color(&colors);
        let alpha = random_range(0.10, 0.95);
        draw.ellipse()
            .xy(circle.xy)
            .w_h(circle.r, circle.r)
            .stroke_weight(0.5)
            .color(rgba(color.red, color.green, color.blue, alpha));

        circles.push(circle);
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