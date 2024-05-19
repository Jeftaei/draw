use crate::modules::dmodifiers::DModifiers;

use super::bindings::TriggerEvents;
use super::windowstate::WindowState;
use super::{bindings::Binding, program::Application};
use winit::event::{DeviceEvent, ElementState, MouseButton};
use winit::keyboard::{Key, KeyCode, PhysicalKey};
use winit::window::WindowId;
use winit::{application::ApplicationHandler, event::WindowEvent};

#[derive(Debug, Clone, Copy)]
pub enum UserEvent {
    WakeUp,
    Redraw,

    MouseInput(Binding<MouseButton>),
    KeyboardInput(Binding<&'static str>),
    StartMinimized,
}

impl ApplicationHandler<UserEvent> for Application {
    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::WakeUp => {}
            UserEvent::Redraw => {
                // dbg!("Requesting redraw");
                self.windows.values_mut().for_each(|window| {
                    window.window.request_redraw();
                });
            }
            UserEvent::KeyboardInput(_) => {}
            UserEvent::MouseInput(_) => {}

            UserEvent::StartMinimized => {
                self.windows.values_mut().for_each(|window| {
                    window.minimize();
                });
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = match self.windows.get_mut(&window_id) {
            None => return,
            Some(w) => w,
        };

        match event {
            WindowEvent::RedrawRequested => {
                let _ = window.present();
            }

            WindowEvent::Resized(size) => window.resize(size),

            WindowEvent::CloseRequested => {
                self.windows.remove(&window_id);
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                window.modifiers = modifiers.state();
            }

            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                let mods = window.modifiers;

                if let Some(action) = Self::process_mouse_binding(button, &mods, state) {
                    self.handle_action(event_loop, window_id, action);
                }
            }

            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let mods = window.modifiers;
                let state = event.state;

                let action = if let Key::Character(ch) = event.logical_key.as_ref() {
                    Self::process_keyboard_binding(&ch.to_uppercase(), &mods, state)
                } else {
                    None
                };

                if let Some(action) = action {
                    self.handle_action(event_loop, window_id, action);
                }
            }

            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                window.cursor_moved(position);
            }

            _ => {}
        }
    }

    // fn device_event(
    //     &mut self,
    //     event_loop: &winit::event_loop::ActiveEventLoop,
    //     device_id: winit::event::DeviceId,
    //     event: winit::event::DeviceEvent,
    // ) {
    //     match event {
    //         DeviceEvent::Button { button, state } => {
    //             dbg!(button, state);
    //         }
    //         DeviceEvent::MouseMotion { delta } => {}
    //         _ => {}
    //     };
    // }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.create_window(event_loop)
            .expect("Failed to init window");
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.windows.is_empty() {
            println!("No windows, exiting...");
            event_loop.exit();
        }
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.context = None;
    }
}
