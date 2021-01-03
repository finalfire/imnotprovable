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

pub fn pick_color(c: &Vec<Rgb>) -> Rgb {
    c[random_range(0, c.len())]
}