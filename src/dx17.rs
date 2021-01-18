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

        if a >= ((x*x) + (y*y)).sqrt() {
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

    ].iter().map(|c| shex2dec(c)).collect();
    let transparent = rgba(1., 1., 1., 0.);

    draw.background().color(shex2dec("F7F3E3"));

    let n = 1000;
    let r_min = 5.0;
    let r_max = 180.0;
    let tries = 1000;

    let half_window = 768. / 2.;
    let boundary = half_window / 4.;

    println!("boundary: {}", boundary);

    /*draw.rect()
        .w_h(-2.*boundary, boundary*2.)
        .color(transparent)
        .stroke_weight(1.0)
        .stroke_color(WHITE);*/

    let quads = [ pt2(-192., 192.), pt2(-192., -192.), pt2(192., 192.), pt2(192., -192.)];

    for quad in quads.iter() {
    
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
                if collision(&circle, &circles, boundary) {
                    circle.r -= 1.0;
                    break;
                }
            }

            let color = pick_color(&colors);
            draw.ellipse()
                .xy(circle.xy + *quad)
                .w_h(circle.r, circle.r)
                //.stroke_weight(0.5)
                .color(color);

            circles.push(circle);
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