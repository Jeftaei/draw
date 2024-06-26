use crate::modules::dmodifiers::DModifiers;

use super::bindings::TriggerEvents;
use super::windowstate::WindowState;
use super::{bindings::Binding, program::Application};
use winit::event::{DeviceEvent, ElementState, MouseButton, MouseScrollDelta, TouchPhase};
use winit::keyboard::{Key, KeyCode, PhysicalKey};
use winit::window::WindowId;
use winit::{application::ApplicationHandler, event::WindowEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayEvent {
    RightClick,

    Exit,
}

impl From<TrayEvent> for UserEvent {
    fn from(v: TrayEvent) -> UserEvent {
        UserEvent::TrayEvent(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserEvent {
    WakeUp,
    Redraw,

    StartMinimized,

    TrayEvent(TrayEvent),
}

impl ApplicationHandler<UserEvent> for Application {
    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::WakeUp => {}
            UserEvent::Redraw => {
                // dbg!("Requesting redraw");
                self.windows.values_mut().for_each(|window| {
                    window.window.request_redraw();
                });
            }

            UserEvent::StartMinimized => {
                self.windows.values_mut().for_each(|window| {
                    window.minimize();
                    window.draw_mode = false
                });
            }

            UserEvent::TrayEvent(t) => match t {
                TrayEvent::RightClick => {
                    self.tray.show_menu().unwrap();
                }

                TrayEvent::Exit => {
                    _event_loop.exit();
                }
            },
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

            WindowEvent::MouseWheel {
                device_id: _,
                delta: MouseScrollDelta::LineDelta(_, y),
                phase: _,
            } => {
                window.canvas.change_brush_size(y as i32);
            }

            //
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                let mods = window.modifiers;

                if let Some(action) = Self::process_mouse_binding(button, &mods, state) {
                    self.handle_action(event_loop, window_id, action);
                }
            }

            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
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
                device_id: _,
                position,
            } => {
                window.cursor_moved(position);
            }

            _ => {}
        }
    }

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        // FIXME
        // Lol, this should be fine since the first window SHOULD ! always be the top level/parent window
        // if for some reason i start using multiple windows, i should fix this...
        let wid = match self.windows.keys().nth(0) {
            Some(wid) => *wid,
            None => return,
        };

        match event {
            DeviceEvent::Button { .. } => {}

            DeviceEvent::Key(rke) => {
                let keypressed = match rke.physical_key {
                    PhysicalKey::Code(k) => match k {
                        KeyCode::AltLeft
                        | KeyCode::AltRight
                        | KeyCode::SuperLeft
                        | KeyCode::SuperRight
                        | KeyCode::ControlLeft
                        | KeyCode::ControlRight
                        | KeyCode::ShiftLeft
                        | KeyCode::ShiftRight => {
                            self.dmods.set(&k, &rke.state);
                            return;
                        }
                        _ => k,
                    },
                    PhysicalKey::Unidentified(_) => {
                        return;
                    }
                };

                // i have a Small feeling i did this in a dumb way, Guess ill figure it out later :D
                match self.keymap.get_mut(&keypressed) {
                    None => {
                        self.keymap.insert(keypressed, rke.state.is_pressed());
                    }
                    Some(v) => match v {
                        false => {
                            if rke.state.is_pressed() {
                                *v = true;
                            }
                        }
                        true => {
                            if rke.state.is_pressed() {
                                return;
                            } else {
                                *v = false
                            }
                        }
                    },
                };

                if let Some(action) =
                    Self::process_device_binding(keypressed, self.dmods, rke.state)
                {
                    self.handle_action(event_loop, wid, action);
                };
            }
            _ => {}
        };
    }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.create_window(event_loop)
            .expect("Failed to init window");
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.windows.is_empty() {
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.context = None;
    }
}
