mod dx;
use dx::{*};

use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};

fn view(app: &App, frame: Frame) {
    // do not write after the first frame
    if app.elapsed_frames() > 1 {
        return;
    }

    // Begin drawing
    let draw = app.draw();

    let colors: Vec<Rgb> = [
        "1c77c3","39a9db","40bcd8","f39237","d63230",
        "fb3640","605f5e","247ba0","e2e2e2"
    ].iter().map(|c| shex2dec(c)).collect();

    draw.background().color(shex2dec("0b2545"));

    let w = 540.;
    let n = 60;
    let m = 40;
    
    let step = w / n as f32;
    let y_off = 13.0;
    let h = m as f32 * y_off;

    let noise = Perlin::new();

    for k in 0..m {
        let mut points: Vec<Point2> = Vec::new();
        let break_point = random_range(1, n);

        let mut last_x = 0.;
        let mut broke = false;

        for i in 0..n {
            if i <= break_point {
                points.push(pt2((i as f32 * step) - (w / 2.), (k as f32 * y_off * -1.) + (h / 2.)));
            } else {
                if !broke {
                    last_x = points.last().unwrap().x;
                    broke = true;
                }
                let mut last_y = points.last().unwrap().y;
                if last_y - y_off < h / 2. {
                    points.push(pt2(last_x, -(h / 2.)));
                    break;
                }
                points.push(pt2(last_x, last_y - y_off));
            }
        }

        let rnd_color = pick_color(&colors);
        let rnd_alpha = random_range(0.25, 0.95);
        let rnd_weight = random_range(0.25, 1.65);

        draw.polyline()
            .weight(random_range(0.15, 1.25))
            .points_colored(points.iter().map(|p| {
                //let x = p.x + random_range(-5.25, 5.25);
                //let y = p.y + random_range(-5.25, 5.25);
                let x = p.x + (random_range(-11.5, 11.5) * noise.get([random_range(-1., 1.) * p.x as f64, random_range(-1., 1.) * p.y as f64, 0.0]) as f32);
                let y = p.y + (random_range(-8.5, 8.5) * noise.get([random_range(-1., 1.) * p.x as f64, random_range(-1., 1.) * p.y as f64, 0.0]) as f32);
                let r_color = pick_color(&colors);
                (pt2(x,y), rgba(r_color.red, r_color.green, r_color.blue, random_range(0.55, 1.0)))
            }));

        if random_f32() > 0.90 {
            draw.polyline()
                .weight(rnd_weight)
                .points_colored(points.iter().map(|p| (*p, rgba(rnd_color.red, rnd_color.green, rnd_color.blue, rnd_alpha))));
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