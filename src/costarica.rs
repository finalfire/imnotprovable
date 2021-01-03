mod dx;
use dx::{*};

use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};

struct Circle {
    xy: Point2,
    r: f32,
    c: Rgba
}

impl Circle {
    fn new(xy: Point2, r: f32, c: Rgba) -> Circle {
        Circle {xy, r, c}
    }
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

    draw.background().color(shex2dec("0E1116"));

    let n = 1580;
    let r_min = 2.55;
    let r_max = 210.0;
    let half_window = 768. / 2.;
    let tries = 1000;

    let boundary = half_window;

    let mut circles: Vec<Circle> = Vec::new();
    for _i in 0..n {
        let color = pick_color(&colors);
        let alpha = 1.;

        let circle = Circle::new(
            pt2(random_range(-boundary, boundary), random_range(-boundary, boundary)),
            r_min,
            rgba(color.red, color.green, color.blue, alpha)
        );
        circles.push(circle);
    }

    /*for circle in circles.iter() {
        let color = pick_color(&colors);
        draw.ellipse()
            .xy(circle.xy)
            .w_h(circle.r, circle.r)
            .color(circle.c);
    }*/

    let noise = Perlin::new();
    for circle in circles.iter() {
        let mut points: Vec<Point2> = Vec::new();
        points.push(circle.xy);

        for _i in 0..1000 {
            let last_point = points.last().unwrap();
            let a = noise.get([last_point.x.sin() as f64, last_point.y.cos() as f64, 0.]) as f32;
            let b = noise.get([last_point.x.cos() as f64, last_point.y.sin() as f64, 1.]) as f32;
            points.push(
                *last_point + pt2(a * 4.23, b * 4.23)
                //*last_point + pt2(last_point.x.sin() * 23.5, last_point.y.sin() * 23.5)
            )
        }

        /*draw.line()
            .start_cap_round()
            .end_cap_round()
            .start(circle.xy)
            .end(pt2(circle.xy.x, circle.xy.y - random_range(-123., 123.)))
            .weight(r_min)
            .color(circle.c);*/
        draw.polyline()
            .color(rgba(circle.c.red, circle.c.green, circle.c.blue, random_range(0.15, 1.0)))
            .weight(random_range(0.1, 2.))
            .points(points);
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