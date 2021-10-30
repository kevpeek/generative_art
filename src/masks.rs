use image::{Pixel, RgbaImage};

pub fn grayscale() -> impl Fn(RgbaImage) -> RgbaImage {
    |mut input| {
        input.pixels_mut().for_each(|pixel| {
            let (r, g, b, a) = pixel.channels4();
            let luminance = (0.21 * r as f32 + 0.72 * g as f32 + 0.07 * b as f32) as u8;
            *pixel = image::Rgba([luminance, luminance, luminance, a]);
        });
        input
    }
}

pub fn circleMask(radius: i32, center_x: i32, center_y: i32) -> impl Fn(RgbaImage) -> RgbaImage {
    move |mut input| {
        for (x, y, pixel) in input.enumerate_pixels_mut() {
            let xx = x as i32 - center_x;
            let yy = y as i32 - center_y;

            let (r, g, b, a) = pixel.channels4();
            let alpha = if xx*xx + yy*yy < radius*radius {a} else {0};
            *pixel = image::Rgba([r, g, b, alpha])
        }

        input
    }
}
