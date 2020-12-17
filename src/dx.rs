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

pub fn shex2deca(c: &str) -> Rgba {
    hex2deca(&c[0..2], &c[2..4], &c[4..6], 1.0)
}

pub fn random_color() -> Rgb {
    rgb(random_f32(), random_f32(), random_f32())
}

pub fn dx7(app: &App, frame: &Frame, draw: &Draw) {
    draw.background().color(shex2dec("fefefe"));
    let colors: Vec<Rgba> = [
        "8eb1c7","b02e0c","eb4511","c1bfb5","fefdff",
        "595f72","575d90","84a07c","c3d350","e6f14a",
        "541388","d90368","f1e9da","2e294e","ffd400"
    ].iter().map(|c| shex2deca(c)).collect();
    
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);
    let stroke_color = shex2dec("333333");

    let size = 32.;

    let start_x = -224.;
    let start_y = 208.;

    let broken = {
        if random_f32() <= 0.25 {
            random_range(0, 15)
        } else { -1 }
    };

    println!("{}", broken);

    for i in 0..15 {

        if i == broken { continue; }

        for j in 0..15 {

            if j == broken { continue; }

            let pm_rot = { if random_f32() < 0.5 { -1 } else { 1 } };
            let pm_dis = { if random_f32() < 0.5 { -1 } else { 1 } };
            
            let ramt = (14-i) as f32 / size * PI / 180.0 * pm_rot as f32 * random_f32() * 28.85;
            let damt = (14-i) as f32 / size * pm_dis as f32 * random_f32() * 14.71;

            let px = start_x + (size * j as f32) + damt;
            let py = start_y - (size * i as f32);

            if random_f32() <= 0.95 {
                // shadow
                if random_f32() <= 0.75 {
                    draw.rect()
                        .x_y(px + 5., py - 5.)
                        .w_h(size, size)
                        .rotate(ramt)
                        .color(rgba(0., 0., 0., random_range(0.10, 0.25)));
                }

                // original
                draw.rect()
                    .x_y(px, py)
                    .w_h(size, size)
                    .rotate(ramt)
                    .color({ if random_f32() <= 0.35 { colors[random_range(0, colors.len())] } else { transparent }})
                    .stroke(stroke_color)
                    .stroke_weight(random_range(0.85, 1.76));

                if random_f32() < 0.25 {
                    draw.rect()
                        .x_y(px, py)
                        .w_h(size, size)
                        .rotate(ramt + random_range(-0.25, 0.25))
                        .color(transparent)
                        .stroke(rgba(0.0, 0.0, 0.0, random_range(0.45, 0.85)))
                        .stroke_weight(random_range(0.33, 1.35));
                }

            }
        }
    }
}

pub fn dx4(app: &App, frame: &Frame, draw: &Draw) {
    draw.background().color(shex2dec("D9E5D6"));
    let colors: Vec<Rgba> = [
        "E8DB7D","28C2FF","BB4430", "473198"//,"231F20"
    ].iter().map(|c| shex2deca(c)).collect();
    
    let transparent = rgba(1.0, 1.0, 1.0, 0.0);
    let stroke_color = shex2dec("333333");

    let size = 32.;

    let start_x = -192.;
    let start_y = 192.;

    for i in 0..13 {
        for j in 0..13 {
            let pm_rot = { if random_f32() < 0.5 { -1 } else { 1 } };
            let pm_dis = { if random_f32() < 0.5 { -1 } else { 1 } };
            
            let ramt = j as f32 / size * PI / 180.0 * pm_rot as f32 * random_f32() * 26.85;
            let damt = j as f32 / size * pm_dis as f32 * random_f32() * 12.21;

            draw.rect()
                .x_y(start_x + (size * j as f32) + damt, start_y - (size * i as f32))
                .w_h(size, size)
                .rotate(ramt)
                .color({ if random_f32() <= 0.35 { colors[random_range(0, colors.len())] } else { transparent }})
                .stroke(stroke_color)
                .stroke_weight(1.75);

            if random_f32() < 0.25 {
                draw.rect()
                    .x_y(start_x + (size * j as f32) + damt, start_y - (size * i as f32))
                    .w_h(size, size)
                    .rotate(ramt + random_range(-0.25, 0.25))
                    .color(transparent)
                    .stroke(rgba(0.0, 0.0, 0.0, random_range(0.45, 0.85)))
                    .stroke_weight(0.98);
            }
        }
    }
}

pub fn dx3(app: &App, frame: &Frame, draw: &Draw) {

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
}

pub fn dx2(app: &App, frame: &Frame, draw: &Draw) {
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