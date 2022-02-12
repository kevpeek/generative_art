// use image::{ImageBuffer, Pixel, Rgba, RgbaImage};
//
// pub fn grayscale() -> impl Fn(RgbaImage) -> RgbaImage {
//     |mut input| {
//         input.pixels_mut().for_each(|pixel| {
//             let (r, g, b, a) = pixel.channels4();
//             let luminance = (0.21 * r as f32 + 0.72 * g as f32 + 0.07 * b as f32) as u8;
//             *pixel = image::Rgba([luminance, luminance, luminance, a]);
//         });
//         input
//     }
// }
//
// pub fn resize(new_width: u32, new_height: u32) -> impl Fn(RgbaImage) -> RgbaImage {
//     move | input | {
//         let scaling_ratio_x = input.width() as f32 / new_width as f32;
//         let scaling_ratio_y = input.height() as f32 / new_height as f32;
//         let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(new_width, new_height);
//         for (x, y, pixel) in output.enumerate_pixels_mut() {
//             let projected_x = (x as f32 * scaling_ratio_x) as u32;
//             let projected_y = (y as f32 * scaling_ratio_y) as u32;
//             *pixel = *input.get_pixel(projected_x, projected_y);
//         }
//         output
//     }
// }
