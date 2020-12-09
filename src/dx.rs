use nannou::prelude::*;

pub fn hex2dec(r: &str, g: &str, b: &str) -> Rgb {
    rgb(u8::from_str_radix(r, 16).unwrap() as f32 / 255.0,
        u8::from_str_radix(g, 16).unwrap() as f32 / 255.0,
        u8::from_str_radix(b, 16).unwrap() as f32 / 255.0)
}

pub fn hex2deca(r: &str, g: &str, b: &str, alpha: f32) -> Rgba {
    let h = hex2dec(r, g, b);
    rgba(h.red, h.green, h.blue, alpha)
}

pub fn shex2dec(c: &str) -> Rgb {
    hex2dec(&c[0..2], &c[2..4], &c[4..6])
}

pub fn dx1(app: &App, frame: &Frame, draw: &Draw) {
    let n = 33;
    let m = 11;
    let w = 417.0;

    let step_x = w / m as f32;
    let step_y = 10.0;

    let half_point_x = (w - step_x) / 2.0;
    let half_point_y = ((n - 1) as f32 * step_y) / 2.0;

    type Point2VV = Vec<Vec<Point2>>;
    let mut lines: Point2VV = Vec::new();

    let background = hex2deca("ed", "b8", "8b", 1.0);
    let foreground = hex2deca("2e", "28", "2a", 1.0);

    draw.background().color(background);

    for i in 0..n {
        let mut points: Vec<Point2<f32>> = Vec::new();

        for p in 0..m {
            let x = (p as f32 * step_x) - half_point_x;
            let mut y = (i as f32 * step_y) - half_point_y;

            if i < n / 2 && p > m / 2 {
                if random_f32() >= 0.25 {
                    y += random_range(-6.5, 7.5);                
                } else {
                    y += random_range(-11.23, 14.23);                
                }
            } else {
                if random_f32() >= 0.65 {
                    y += random_range(-1.89, 1.89);
                } 
            }

            points.push(pt2(x,y));
        }

        draw.polyline()
            .weight(1.0)
            .points_colored(points.iter().map(|p| (*p, foreground)));

        for k in 0..5 {
            let shadows = points.iter()
                .map(|p| {
                    let y = p.y + random_range(-1.23, 1.23);
                    pt2(p.x, y)
                });
            
            draw.polyline()
                .weight(0.8)
                .points_colored(shadows.map(|p| { 
                    (*p, rgba(foreground.red, foreground.green, foreground.blue, random_range(0.05, 0.45))
                )}));
        }
        lines.push(points);
    }
}

pub fn dx0(app: &App, frame: &Frame, draw: &Draw) {
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
}