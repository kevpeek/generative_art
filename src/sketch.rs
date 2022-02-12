use std::f32::consts::PI;
use image::{Pixel, Rgba, RgbaImage};
use imageproc::drawing::{Blend, Canvas, draw_line_segment_mut, draw_polygon_mut};
use imageproc::point::Point;
use rand::Rng;

#[derive(Clone)]
pub struct UserParams {
    pub destination_width: u32,
    pub destination_height: u32,
    pub stroke_ratio: f32,
    pub stroke_reduction: f32,
    pub stroke_jitter: f32,
    pub stroke_inversion_threshold: f32,
    pub initial_alpha: f64,
    pub alpha_increase: f64,
    pub min_edge_count: u32,
    pub max_edge_count: u32,
}

impl Default for UserParams {
    fn default() -> Self {
        let destination_width = 3840;
        UserParams {
            destination_width,
            destination_height: 2160,
            stroke_ratio: 0.75,
            stroke_reduction: 0.002,
            stroke_jitter: (0.1 * destination_width as f64) as f32,
            stroke_inversion_threshold: 0.05,
            initial_alpha: 0.1,
            alpha_increase: 0.006,
            min_edge_count: 3,
            max_edge_count: 4,
        }
    }
}

pub struct Sketch {
    params: UserParams,
    source: RgbaImage,
    destination: Blend<RgbaImage>,
    source_width: u32,
    source_height: u32,
    x_scale: f32,
    y_scale: f32,
    stroke_size: f32,
    initial_stroke_size: f32,
}

impl Sketch {
    pub fn new(source: RgbaImage, params: UserParams) -> Sketch {
        let stroke_size = params.stroke_ratio * params.destination_width as f32;
        let (source_width, source_height) = source.dimensions();

        let x_scale = params.destination_width as f32 / source_width as f32;
        let y_scale = params.destination_height as f32 / source_height as f32;
        Sketch {
            params: params.clone(),
            source,
            destination: Blend(RgbaImage::new(params.destination_width, params.destination_height)),
            source_width,
            source_height,
            x_scale,
            y_scale,
            stroke_size,
            initial_stroke_size: stroke_size,
        }
    }

    pub fn run_cycles(&mut self, number_of_cycles: usize) {
        for _ in 0..number_of_cycles {
            self.update();
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        let source_x = rng.gen_range(0..self.source_width);
        let source_y = rng.gen_range(0..self.source_height);

        let destination_x = source_x as f32 * self.x_scale + rng.gen_range(-self.params.stroke_jitter..self.params.stroke_jitter);
        let destination_y = source_y as f32 * self.y_scale + rng.gen_range(-self.params.stroke_jitter..self.params.stroke_jitter);
        let destination_point = (destination_x, destination_y);

        let edge_count = rng.gen_range(self.params.min_edge_count..=(self.params.max_edge_count));

        let mut color = self.source.get_pixel(source_x, source_y).to_owned();
        color.0[3] = self.params.initial_alpha as u8;

        let polygon_points = regular_polygon_points(destination_point, self.stroke_size, edge_count, rng.gen());
        draw_polygon_mut(&mut self.destination, &polygon_points, color);

        if let Some(edge_color) = self.choose_edge_color(&mut color) {
            // Append the first point to the end in order to get a closed shape.
            let mut polygon_points= polygon_points;
            polygon_points.push(*polygon_points.first().unwrap());
            draw_hollow_polygon_mut(&mut self.destination, &polygon_points, edge_color);
        }

        self.stroke_size -= self.params.stroke_reduction * self.stroke_size;
        self.params.initial_alpha += self.params.alpha_increase;
    }

    fn choose_edge_color(&mut self, color: &mut Rgba<u8>) -> Option<Rgba<u8>> {
        if self.stroke_size <= self.params.stroke_inversion_threshold * self.initial_stroke_size {
            let (r, g, b, _) = color.channels4();
            let sum = r as u32 + g as u32 + b as u32;
            if sum / 3 < 128 {
                Some(Rgba([255, 255, 255, (self.params.initial_alpha * 2.0) as u8]))
            } else {
                Some(Rgba([0, 0, 0, (self.params.initial_alpha * 2.0) as u8]))
            }
        } else {
            None
        }
    }

    pub fn output(self) -> RgbaImage {
        self.destination.0.clone()
    }
}


pub fn draw_hollow_polygon_mut<C>(canvas: &mut C, poly: &[Point<i32>], color: C::Pixel)
    where
        C: Canvas,
        C::Pixel: 'static,
{
    for i in 0..(poly.len() - 1) {
        draw_line_segment_mut(
            canvas,
            (poly[i].x as f32, poly[i].y as f32),
            (poly[i + 1].x as f32, poly[i + 1].y as f32),
            color,
        )
    }
}

/// Generates points for a regular polygon
pub fn regular_polygon_points(
    position: (f32, f32),
    radius: f32,
    sides: u32,
    rotation: f32,
) -> Vec<Point<i32>> {
    let (x, y) = position;

    let angle = 2.0 * PI / sides as f32;
    let points: Vec<Point<i32>> = (0..sides).into_iter()
        .map(|n| Point::new(
            (x + radius * (angle * n as f32 + rotation).cos()) as i32,
            (y + radius * (angle * n as f32 + rotation).sin()) as i32,
        )).collect();

    if points.first() == points.last() {
        // When the shape gets really small, you may just get the same point. Return a dummy value.
        return vec![
            Point::new(x as i32, y as i32),
            Point::new(x as i32 + 1, y as i32 + 1),
        ];
    }

    points
}
