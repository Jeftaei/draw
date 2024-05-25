use crate::modules::dmodifiers::DModifiers;

use super::apphandler::UserEvent;
use super::bindings::{DEVICE_BINDINGS, KEYBOARD_BINDINGS};
use super::windowstate::WindowState;
use super::{actions::Actions, bindings::MOUSE_BINDINGS};

use softbuffer::Context;
use std::{collections::HashMap, error::Error};
use trayicon::TrayIcon;
use wgpu::rwh::{DisplayHandle, HasDisplayHandle};
use winit::event::ElementState;
use winit::keyboard::KeyCode;
use winit::platform::windows::WindowExtWindows;
use winit::window::Fullscreen;
use winit::{
    event::MouseButton,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::ModifiersState,
    window::{CustomCursor, Icon, WindowId},
    window::{Window, WindowAttributes, WindowLevel},
};

pub struct Application {
    // app_icon: Icon,

    // cursors: HashMap<String, CustomCursor>,
    pub windows: HashMap<WindowId, WindowState>,

    pub context: Option<Context<DisplayHandle<'static>>>,

    pub dmods: DModifiers,

    pub keymap: HashMap<KeyCode, bool>,

    pub tray: TrayIcon<UserEvent>,
}

impl Application {
    pub fn new<T>(event_loop: &EventLoop<T>, tray: TrayIcon<UserEvent>) -> Self {
        // we HAVE to drop the context right before the event loop stops, or else we will fucking LEAK memory !
        let context = Some(
            Context::new(unsafe {
                std::mem::transmute::<DisplayHandle<'_>, DisplayHandle<'static>>(
                    event_loop.display_handle().unwrap(),
                )
            })
            .unwrap(),
        );

        // let icon =

        Self {
            context,
            tray,

            windows: Default::default(),

            dmods: Default::default(),

            keymap: Default::default(),
        }
    }

    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) -> Result<WindowId, Box<dyn Error>> {
        let window_attributes = Window::default_attributes()
            .with_title("test_window")
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            // .with_visible(false)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_decorations(false)
            .with_transparent(true);

        let window = event_loop.create_window(window_attributes)?;
        window.set_skip_taskbar(true);

        let window_state = WindowState::new(self, window)?;
        let window_id = window_state.window.id();

        self.windows.insert(window_id, window_state);

        Ok(window_id)
    }

    pub fn handle_action(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: WindowId,
        action: Actions,
    ) {
        let window = self.windows.get_mut(&window_id).unwrap();

        match action {
            Actions::CloseWindow => {
                let _ = self.windows.remove(&window_id);
            }

            Actions::Minimize => {
                window.minimize();
            }

            Actions::ToggleMaximize => {}

            Actions::ToggleDecorations => {
                window.toggle_decoration();
            }

            Actions::ToggleFullscreen => {
                window.toggle_fullscreen();
            }

            // yeah these feel repetitive but i think it could be useful for something at somepoint
            Actions::ToggleDrawMode => match window.draw_mode {
                true => {
                    window.exit_draw_mode();
                }
                false => {
                    window.enter_draw_mode();
                }
            },

            Actions::EnterDrawMode => {
                if !window.draw_mode {
                    window.enter_draw_mode();
                }
            }

            Actions::ExitDrawMode => {
                if window.draw_mode {
                    window.exit_draw_mode();
                }
            }

            Actions::RedoDraw => {
                window.canvas.redo();
            }

            Actions::UndoDraw => {
                window.canvas.undo();
            }

            Actions::SetDrawing => {
                window.invert_drawing();
            }
        }
    }

    pub fn process_mouse_binding(
        button: MouseButton,
        mods: &ModifiersState,
        state: ElementState,
    ) -> Option<Actions> {
        MOUSE_BINDINGS.iter().find_map(|binding| {
            binding
                .is_triggered_by(&button, mods, &state)
                .then_some(binding.action)
        })
    }

    pub fn process_keyboard_binding(
        key: &str,
        mods: &ModifiersState,
        state: ElementState,
    ) -> Option<Actions> {
        KEYBOARD_BINDINGS.iter().find_map(|binding| {
            binding
                .is_triggered_by(&key, mods, &state)
                .then_some(binding.action)
        })
    }

    pub fn process_device_binding(
        key: KeyCode,
        mods: DModifiers,
        state: ElementState,
    ) -> Option<Actions> {
        DEVICE_BINDINGS.iter().find_map(|binding| {
            binding
                .is_triggered_by(&key, &mods.into(), &state)
                .then_some(binding.action)
        })
    }
}
