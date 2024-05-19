use winit::{
    event::ElementState,
    keyboard::{KeyCode, ModifiersState},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct DModifiers {
    pub shift: bool,
    pub control: bool,
    pub _super: bool,
    pub alt: bool,
}

impl DModifiers {
    pub fn new() -> Self {
        Self {
            shift: false,
            control: false,
            _super: false,
            alt: false,
        }
    }

    pub fn set(&mut self, kc: &KeyCode, state: &ElementState) {
        match kc {
            // yeah yeah i know i could work around this and not match this twice but.
            // Whatever.
            KeyCode::AltLeft | KeyCode::AltRight => {
                if let ElementState::Pressed = state {
                    self.alt = true;
                } else {
                    self.alt = false;
                }
            }
            KeyCode::SuperLeft | KeyCode::SuperRight => {
                if let ElementState::Pressed = state {
                    self._super = true;
                } else {
                    self._super = false;
                }
            }
            KeyCode::ControlLeft | KeyCode::ControlRight => {
                if let ElementState::Pressed = state {
                    self.control = true;
                } else {
                    self.control = false;
                }
            }
            KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                if let ElementState::Pressed = state {
                    self.shift = true;
                } else {
                    self.shift = false;
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn get(&mut self, k: impl Into<String>) -> bool {
        match k.into().to_uppercase().as_str() {
            "SHIFT" => self.shift,
            "CONTROL" | "CTRL" => self.control,
            "ALT" => self.alt,
            "SUPER" => self._super,
            _ => panic!("How are you This Stupid."),
        }
    }
}

impl From<DModifiers> for ModifiersState {
    fn from(value: DModifiers) -> Self {
        let mut bitflags = 0b0000_0000;

        if value.shift {
            bitflags |= 0b100;
        }

        if value.control {
            bitflags |= 0b100 << 3;
        }

        if value.alt {
            bitflags |= 0b100 << 6;
        }

        if value._super {
            bitflags |= 0b100 << 9;
        }

        ModifiersState::from_bits(bitflags).expect("bad bit values")
    }
}
