mod dx;
use dx::{*};

use nannou::prelude::*;

#[derive(Copy,Clone)]
struct Dot {
    xy: Point2,
    color: Rgb
}

fn view(app: &App, frame: Frame) {
    // do not write after the first frame
    if app.elapsed_frames() > 1 {
        return;
    }

    // Begin drawing
    let draw = app.draw();
    
    let transparent = rgba(1., 1., 1., 0.);
    let background = shex2dec("FFF8F0");
    let border = shex2dec("333333");
    
    let colors: Vec<Rgb> = [
        "f18f01","048ba8","2e4057","99c24d","2f2d2e",
        "0d3b66","f4d35e","ee964b","f95738"
    ].iter().map(|c| shex2dec(c)).collect();

    draw.background().color(background);

    let n = 50;
    let m = 12;
    let off = 12.;
    let space = 38.;
    let h = 768.;
    let nhalf = (n-1) as f32 * off / 2.;
    let mhalf = (m-1) as f32 * space / 2.;
    let w = 2.4;

    let mut dots: Vec<Vec<Dot>> = vec![Vec::new(); n];
    for j in 0..m {
        for i in 0..n {
                if random_f32() > 0.65 {
                    let x = i as f32  * off - nhalf + random_range(-33., 33.);
                    let y = j as f32 * space - mhalf + random_range(-23., 23.);
                    let xy = pt2(x,y);
                    let color = pick_color(&colors);

                    dots[i].push(Dot{ xy, color });
                }
        }
    }
    for i in 0..n {
        dots[i].reverse();
    }

    let co: Vec<Point2> = dots[0].iter().map(|d| d.xy).collect();
    println!("{:?}", co);

    let sizes: Vec<usize> = dots.iter().map(|d| d.len()).collect();
    println!("{:?}", sizes);

    for l in 0..n {
        let start_whole = pt2((l as f32 * off) - nhalf, h / 2.);
        let end_whole = pt2((l as f32 * off) - nhalf, -h / 2.);

        /*draw.line()
            .start(start_whole)
            .end(end_whole)
            .weight(w)
            .color(border);*/
        
        if dots[l].len() > 0 {
            let mut dot_i = 0;
            while dot_i < dots[l].len() {
                let start = dots[l][dot_i].xy;
                let end = if dot_i + 1 < dots[l].len() { dots[l][dot_i+1].xy } else { end_whole };

                draw.line()
                    .start(start)
                    .end(end)
                    .weight(w)
                    .color(dots[l][dot_i].color);
                
                dot_i += 1;
            }
        }
    }

    for dot_family in dots {
        for dot in dot_family {
            draw.ellipse()
                .xy(dot.xy)
                .w_h(18., 18.)
                .color(dot.color)
                .stroke(border)
                .stroke_weight(w);
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