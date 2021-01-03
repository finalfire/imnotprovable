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

    draw.background().color(shex2dec("042A2B"));
    let colors: Vec<Rgb> = [
        "1c77c3","39a9db","40bcd8","f39237","d63230",
        "fb3640","605f5e","247ba0","e2e2e2"
    ].iter().map(|c| shex2dec(c)).collect();
    
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);

    let size = 32.;

    let start_x = -192.;
    let start_y = 192.;
    
    // shear in x and y
    let shear_x = |p: Point2, f: f32| -> Point2 {
        pt2(p.x + f * p.y, p.y)
    };
    let shear_y = |p: Point2, f: f32| -> Point2 {
        pt2(p.x, p.x*f + p.y)
    };

    /*let p1 = pt2(0.0, 0.0);
    let p2 = pt2(100.0, 0.0);
    let p3 = pt2(100.0, 100.0);
    let p4 = pt2(0.0, 100.0);

    draw.quad()
        .color(STEELBLUE)
        .points(p1, p2, p3, p4);

    draw.quad()
        .color(ORANGE)
        .points(shear_x(p1, 0.5), shear_x(p2, 0.5), shear_x(p3, 0.5), shear_x(p4, 0.5));*/

    for i in 0..13 {
        for j in 0..13 {
            let x = start_x + (size * j as f32);
            let y = start_y - (size * i as f32);

            let p1 = pt2(x - size / 2., y - size / 2.);
            let p2 = pt2(x + size / 2., y - size / 2.);
            let p3 = pt2(x + size / 2., y + size / 2.);
            let p4 = pt2(x - size / 2., y + size / 2.);

            let rsx = random_range(-0.009, 0.009);
            let rsy = random_range(-0.009, 0.009);

            let c = pick_color(&colors);
            
            draw.quad()
                .color(transparent)
                .stroke_color(rgba(c.red, c.green, c.blue, 0.75))
                .stroke_weight(0.85)
                .yaw(random_range(-0.525, 0.525))
                .points(
                    /*shear_x(shear_y(p1, rsy), rsx),
                    shear_x(shear_y(p2, rsy), rsx),
                    shear_x(shear_y(p3, rsy), rsx),
                    shear_x(shear_y(p4, rsy), rsx)*/
                    p1, p2, p3, p4
                );
                
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