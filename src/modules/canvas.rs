use softbuffer::Surface;
use std::{
    error::Error,
    num::NonZeroU32,
    sync::Arc,
    time::{Duration, Instant},
};
use wgpu::rwh::DisplayHandle;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

use crate::app::program::Application;
use crate::art::numbers::get_art;

const DEFAULT_BG_COLOR: u32 = 0xff181818;
const CLEAR_BG_COLOR: u32 = 0x00000000;
const DEFAULT_BRUSH_COLOR: u32 = 0xffffccaa;

const BRUSH_SIZE: u32 = 2;

pub struct Canvas {
    pub surface: Surface<DisplayHandle<'static>, Arc<Window>>,

    pub brush_size: u32,
    brush_changed_at: Instant,
    pub brush_color: u32,

    pub canvas_size: PhysicalSize<u32>,
    // zoom feature

    // pan feature
}

impl Canvas {
    pub fn new(app: &Application, window: Arc<Window>) -> Result<Self, Box<dyn Error>> {
        let c_size = window.inner_size();
        let surface = Surface::new(app.context.as_ref().unwrap(), window)?;

        Ok(Self {
            surface,
            brush_size: BRUSH_SIZE,
            brush_changed_at: Instant::now(),
            brush_color: DEFAULT_BRUSH_COLOR,
            canvas_size: c_size,
        })
    }

    pub fn present(&mut self) -> Result<(), Box<dyn Error>> {
        let buf = self.surface.buffer_mut()?;
        buf.present().expect("failed to present buffer");

        Ok(())
    }

    pub fn resize_canvas(&mut self, size: PhysicalSize<u32>) {
        let (width, height) = match (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) {
            (Some(width), Some(height)) => (width, height),
            _ => return,
        };

        self.canvas_size = size;

        // TODO: figure out how to resize this and have the new area be filled in with a default color
        // without overwriting the rest of the canvas
        self.surface
            .resize(width, height)
            .expect("failed to resize canvas surface");
    }

    pub fn fill(&mut self, color: Option<u32>) -> Result<(), Box<dyn Error>> {
        let mut buffer = self.surface.buffer_mut()?;
        buffer.fill(color.unwrap_or(CLEAR_BG_COLOR));

        println!("filling");

        Ok(())
    }

    pub fn change_brush_size(&mut self, d: i32) {
        let original = self.brush_size;

        let n = (self.brush_size as i32 + d).clamp(1, 10) as u32;

        // technically this would also trigger even if the original is the same, but its been 5 seconds so its whatever
        if self.brush_changed_at.elapsed() < Duration::new(5, 0) || original == n {
            return;
        };

        self.brush_size = n;

        let _ = self.draw_number_in_corner(n, None);
    }

    pub fn draw_number_in_corner(
        &mut self,
        n: u32,
        offset: Option<PhysicalPosition<u32>>,
    ) -> Result<(), Box<dyn Error>> {
        // offset is how far away from the borders we want to be
        let offset = offset.unwrap_or(PhysicalPosition { x: 20, y: 100 });
        let num = n;

        let size = self.canvas_size;

        // 16x16 being the pixel size of the number
        let x_start = size.width - offset.x - 16;
        let y_start = size.height - offset.y - 16;

        let mut coords = Vec::new();

        get_art(num).iter().enumerate().for_each(|(i, v)| {
            let y_offset = (i / 16) as u32;

            if v == &1 {
                coords.push((
                    (x_start + (i as u32) % 16) as i32,
                    (y_start + y_offset) as i32,
                ));
            }
        });

        let pixels = self.bulk_pixel_convert(coords);

        let mut buffer = self
            .surface
            .buffer_mut()
            .expect("hopefully this doesnt fail ever !");
        for px in pixels {
            // dbg!(px);
            buffer[px as usize] = self.brush_color;
        }

        Ok(())
    }

    pub fn to_pixel_pos(&self, location: PhysicalPosition<f64>) -> u32 {
        let x = (location.x as u32).clamp(0, self.canvas_size.width - 1);
        let y = (location.y as u32).clamp(0, self.canvas_size.height - 1);

        x + (y * self.canvas_size.width)
    }

    pub fn bulk_pixel_convert(&self, v: Vec<(i32, i32)>) -> Vec<u32> {
        v.iter()
            .map(|(x, y)| {
                let x = (*x as u32).clamp(0, self.canvas_size.width - 1);
                let y = (*y as u32).clamp(0, self.canvas_size.height - 1);

                x + (y * self.canvas_size.width)
            })
            .collect::<Vec<u32>>()
    }

    pub fn get_line_points(
        &self,
        mut prev: PhysicalPosition<f64>,
        curr: PhysicalPosition<f64>,
    ) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = vec![];

        if (prev.x - curr.x).abs() + (prev.y - curr.y).abs() <= 2.0 {
            prev = curr;
        };

        let (x1, y1) = (prev.x as i32, prev.y as i32);
        let (x2, y2) = (curr.x as i32, curr.y as i32);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;
        let mut x = x1;
        let mut y = y1;

        loop {
            points.push((x, y));

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }

        points
    }

    pub fn get_circle_points(&self, location: PhysicalPosition<f64>) -> Vec<(i32, i32)> {
        let radius = self.brush_size as i32 - 1;
        let mut points = Vec::<(i32, i32)>::new();

        let x_center = location.x as i32;
        let y_center = location.y as i32;

        // i still want to draw 1 pixel sometimes
        if self.brush_size == 1 {
            return vec![(x_center, y_center)];
        };

        // Thanks stackoverflow, i dont like math.
        // Very Bad "Algorithm"
        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y < radius * radius + radius {
                    points.push((x_center + x, y_center + y));
                }
            }
        }

        // sort vec and then call dedup, which removes all consecutive repeating values :D
        points.sort();
        points.dedup();
        points
    }

    pub fn draw(
        &mut self,
        location: PhysicalPosition<f64>,
        prev_location: PhysicalPosition<f64>,
    ) -> Result<(), Box<dyn Error>> {
        let line_points = self.get_line_points(prev_location, location);

        let mut points: Vec<u32> = Vec::new();

        for p in line_points {
            points.extend(self.bulk_pixel_convert(self.get_circle_points(p.into())));
        }

        let mut buffer = self.surface.buffer_mut()?;
        for px in points {
            // dbg!(px);
            buffer[px as usize] = self.brush_color;
        }

        Ok(())
    }
}
