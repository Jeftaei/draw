use std::{error::Error, num::NonZeroU32, sync::Arc};

use super::program::Application;
use crate::modules::canvas::Canvas;

use softbuffer::Surface;
use wgpu::rwh::DisplayHandle;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    keyboard::ModifiersState,
    window::{Fullscreen, Window},
};

#[derive(Debug)]
pub struct CursorPos {
    pub current: Option<PhysicalPosition<f64>>,
    pub previous: Option<PhysicalPosition<f64>>,
}

impl CursorPos {
    pub fn new() -> Self {
        Self {
            current: None,
            previous: None,
        }
    }

    pub fn set(&mut self, v: Option<PhysicalPosition<f64>>) {
        self.previous = self.current;
        self.current = v;
    }

    pub fn get(&self) -> (Option<PhysicalPosition<f64>>, Option<PhysicalPosition<f64>>) {
        (self.current, self.previous)
    }
}

pub struct WindowState {
    /// Surface must be dropped before window
    pub canvas: Canvas,

    pub window: Arc<Window>,

    pub cursor_pos: CursorPos,

    pub modifiers: ModifiersState,

    pub zoom: f64,

    pub panned: PhysicalPosition<f32>,

    pub drawing: bool,

    pub draw_mode: bool,
}

impl WindowState {
    pub fn new(app: &Application, window: Window) -> Result<Self, Box<dyn Error>> {
        let window = Arc::new(window);

        let canvas = Canvas::new(app, window.clone())?;

        window.set_ime_allowed(false);

        let size = window.inner_size();
        let mut state = Self {
            canvas,
            window,

            cursor_pos: CursorPos::new(),

            modifiers: Default::default(),
            zoom: Default::default(),
            panned: Default::default(),

            drawing: false,
            draw_mode: false,
        };

        state.resize(size);
        Ok(state)
    }

    pub fn invert_drawing(&mut self) {
        self.drawing = !self.drawing;

        let _ = self.draw_at_cursor();
    }

    pub fn enter_draw_mode(&mut self) {
        self.draw_mode = true;

        self.window.set_minimized(false);
        self.window
            .set_fullscreen(Some(Fullscreen::Borderless(None)));
    }

    pub fn exit_draw_mode(&mut self) {
        self.draw_mode = false;

        self.window.set_fullscreen(None);
        self.window.set_minimized(true);
    }

    pub fn minimize(&mut self) {
        self.window.set_minimized(true);
    }

    pub fn cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_pos.set(Some(position));

        if self.drawing {
            let _ = self.draw_at_cursor();
        };
    }

    pub fn cursor_left(&mut self) {
        self.cursor_pos.set(None);
    }

    pub fn toggle_decoration(&self) {
        let decorated = self.window.is_decorated();
        self.window.set_decorations(!decorated);
    }

    pub fn toggle_fullscreen(&self) {
        let fullscreen = match self.window.fullscreen() {
            Some(_) => None,
            None => Some(Fullscreen::Borderless(None)),
        };

        self.window.set_fullscreen(fullscreen)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.canvas.resize_canvas(size);

        // when the window is resized we fill the buffer with default color again
        // should probably not do this, will figure out later
        let _ = self.canvas.fill(None);
    }

    pub fn draw_at_cursor(&mut self) -> Result<(), Box<dyn Error>> {
        match self.cursor_pos.get() {
            // if theres no current position then we either havent got a cursor moved event,
            // or we have left the window
            (None, _) => Ok(()),

            (Some(pos), Some(pos2)) => {
                self.canvas.draw(pos, pos2).expect("failed to draw");

                Ok(())
            }

            (Some(pos), _) => {
                self.canvas.draw(pos, pos).expect("failed to draw");

                Ok(())
            }
        }
    }

    pub fn present(&mut self) -> Result<(), Box<dyn Error>> {
        self.window.pre_present_notify();

        let _ = self.canvas.present();

        Ok(())
    }
}
