use softbuffer::Surface;
use std::{error::Error, num::NonZeroU32, sync::Arc};
use wgpu::rwh::DisplayHandle;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

use crate::app::program::Application;

const DEFAULT_BG_COLOR: u32 = 0xff181818;
const CLEAR_BG_COLOR: u32 = 0x00000000;
const DEFAULT_BRUSH_COLOR: u32 = 0xffffccaa;

pub struct Canvas {
    pub surface: Surface<DisplayHandle<'static>, Arc<Window>>,

    pub brush_size: u32,
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
            brush_size: 1,
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

    pub fn to_pixel_pos(location: PhysicalPosition<f64>, size: PhysicalSize<u32>) -> u32 {
        let x = (location.x as u32).clamp(0, size.width - 1);
        let y = (location.y as u32).clamp(0, size.height - 1);

        x + (y * size.width)
    }

    pub fn get_cell_neighbors(
        location: PhysicalPosition<f64>,
        canvas_size: PhysicalSize<u32>,
        brush_size: u32,
    ) -> Vec<u32> {
        // this code sucks, also not really want i want tbh

        // let mut neighbors = vec![];

        // let (x, y) = (location.x as u32, location.y as u32);
        // let (x_bound, y_bound) = (canvas_size.width - 1, canvas_size.height - 1);

        // for dx in (x.saturating_sub(brush_size))..=(x.saturating_add(brush_size)) {
        //     for dy in (y.saturating_sub(brush_size))..=(y.saturating_add(brush_size)) {
        //         if dx > x_bound || dy > y_bound {
        //             continue;
        //         }
        //         // dbg!(dx, dy, x_bound, y_bound, x, y);

        //         neighbors.push(Self::pixel_from_coord(dx, dy, canvas_size.width));
        //     }
        // }

        // neighbors
        vec![Self::to_pixel_pos(location, canvas_size)]
    }

    pub fn draw(&mut self, location: PhysicalPosition<f64>) -> Result<(), Box<dyn Error>> {
        let mut buffer = self.surface.buffer_mut()?;

        let pxls: Vec<u32> = Self::get_cell_neighbors(location, self.canvas_size, self.brush_size);

        for px in pxls {
            // dbg!(px);
            buffer[px as usize] = self.brush_color;
        }

        Ok(())
    }
}
