mod dx;
use dx::{*};

use std::collections::HashSet;
use nannou::prelude::*;

enum Axis { X, Y }

#[derive(Clone,Copy)]
struct Square {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

fn split_x(square: &Square, v: f32, squares: &mut Vec<Square>) {
    // split square on the x axis at v point
    // it produces two rects a,b
    let a = Square {
        x: square.x,
        y: square.y,
        w: square.w - (square.w - v + square.x),
        h: square.h
    };

    let b = Square {
        x: v,
        y: square.y,
        w: square.w - v + square.x,
        h: square.h
    };

    squares.push(a);
    squares.push(b);
}

fn split_y(square: &Square, v: f32, squares: &mut Vec<Square>) {
    // split square on the x axis at v point
    // it produces two rects a,b
    let a = Square {
        x: square.x,
        y: square.y,
        w: square.w,
        h: square.h - (square.h - v + square.y)
    };

    let b = Square {
        x: square.x,
        y: v,
        w: square.w,
        h: square.h - v + square.y
    };

    squares.push(a);
    squares.push(b);
}

fn split_on(axis: Axis, v: f32, s: &mut Vec<Square>) {
    let mut i: i32 = (s.len() - 1) as i32;
    while i >= 0 {
        // get the last square
        let q = s[i as usize];

        // split on specificed axis
        // each split removes the square and produces two new rects
        match axis {
            Axis::X => {
                //if v > q.x - (q.w / 2.) && v < q.x + (q.w / 2.) {
                if v > q.x && v < q.x + q.w {
                    if random_f32() > 0.5 {
                        let o = s.remove(i as usize);
                        split_x(&o, v, s);
                    }
                }
            },
            Axis::Y => {
                //if v > q.y - (q.h / 2.) && v < q.y + (q.h / 2.) {
                if v > q.y && v < q.y + q.h {
                    if random_f32() > 0.5 {
                        let o = s.remove(i as usize);
                        split_y(&o, v, s);
                    }
                }
            }
        }

        i -= 1;
    }
}

fn view(app: &App, frame: Frame) {
    // do not write after the first frame
    if app.elapsed_frames() > 1 {
        return;
    }

    // Begin drawing
    let draw = app.draw();
    
    let transparent = rgba(1., 1., 1., 0.);
    let background = shex2dec("0D1F2D");
    let border = shex2dec("F4F4F6");
    let colors: Vec<Rgb> = [
        "f18f01","048ba8","2e4057","99c24d",
        "3e6990","aabd8c","e9e3b4","f39b6d"
    ].iter().map(|c| shex2dec(c)).collect();

    draw.background().color(background);

    let size = 480.;
    let step = size / 5.;
    let colored_perc = 0.75;

    let mut squares: Vec<Square> = Vec::new();
    squares.push(
        Square {x: 0., y: 0., w: size, h: size}
    );

    for i in (0..size as usize).step_by(step as usize) {
        split_on(Axis::X, i as f32, &mut squares);
        split_on(Axis::Y, i as f32, &mut squares);
    }

    println!("n squares: {}", squares.len());

    let mut square_color: Vec<Rgba> = vec![transparent; squares.len()];
    let mut yet_to_color = (squares.len() as f32 * colored_perc).round() as usize;
    println!("to color: {}", yet_to_color);
    let mut last_one = random_range(0, squares.len());
    while yet_to_color > 0 && square_color[last_one] == transparent {
        let c = pick_color(&colors);
        square_color[last_one] = rgba(c.red, c.green, c.blue, 1.);
        last_one = random_range(0, squares.len());
        yet_to_color -= 1;
    }

    for (i, s) in squares.iter().enumerate() {
        let points: Vec<Point2> = [
            pt2(s.x, s.y),
            pt2(s.x + s.w, s.y),
            pt2(s.x + s.w, s.y + s.h),
            pt2(s.x, s.y + s.h)
        ].iter().map(|p| pt2(p.x - size / 2., p.y - size / 2.)).collect();

        draw.quad()
            .points(points[0], points[1], points[2], points[3])
            .stroke_weight(3.5)
            .stroke_color(border)
            .color(square_color[i]);
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