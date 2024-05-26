use super::actions::Actions;
use winit::{
    event::{ElementState, MouseButton},
    keyboard::{KeyCode, ModifiersState},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerEvents {
    Toggle,
    OneTime(ElementState),
}

impl TriggerEvents {
    pub fn inner(&self) -> &ElementState {
        match self {
            Self::Toggle => unreachable!(),
            Self::OneTime(i) => i,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Binding<T: Eq> {
    trigger: T,
    mods: Option<ModifiersState>,

    conditions: TriggerEvents,

    pub action: Actions,

    is_pressed: bool,
}

impl<T: Eq> Binding<T> {
    const fn new(
        trigger: T,
        mods: Option<ModifiersState>,
        action: Actions,
        conditions: TriggerEvents,
    ) -> Self {
        Self {
            trigger,
            mods,
            action,
            conditions,
            is_pressed: false,
        }
    }

    pub fn is_triggered_by(
        &self,
        trigger: &T,
        mods: &ModifiersState,
        state: &ElementState,
    ) -> bool {
        let condition = match self.conditions {
            TriggerEvents::Toggle => true,
            TriggerEvents::OneTime(evnt) => &evnt == state,
        };

        let mods_condition = match self.mods {
            None => {
                // Allow any set of modifiers to pass if not explicitly set
                true
            }
            Some(m) => &m == mods,
        };

        &self.trigger == trigger && mods_condition && condition
    }
}

pub const MOUSE_BINDINGS: &[Binding<MouseButton>] = &[Binding::new(
    MouseButton::Left,
    None,
    Actions::SetDrawing,
    TriggerEvents::Toggle,
)];

pub const KEYBOARD_BINDINGS: &[Binding<&'static str>] = &[
    Binding::new(
        "C",
        Some(ModifiersState::CONTROL),
        Actions::CloseWindow,
        TriggerEvents::OneTime(ElementState::Pressed),
    ),
    Binding::new(
        "ESC",
        Some(ModifiersState::empty()),
        Actions::ExitDrawMode,
        TriggerEvents::OneTime(ElementState::Pressed),
    ),
    Binding::new(
        "Z",
        Some(ModifiersState::CONTROL),
        Actions::UndoDraw,
        TriggerEvents::OneTime(ElementState::Pressed),
    ),
    Binding::new(
        "Y",
        Some(ModifiersState::CONTROL),
        Actions::RedoDraw,
        TriggerEvents::OneTime(ElementState::Pressed),
    ),
];

pub const DEVICE_BINDINGS: &[Binding<KeyCode>] = &[Binding::new(
    KeyCode::KeyD,
    Some(ModifiersState::CONTROL.union(ModifiersState::ALT)),
    Actions::ToggleDrawMode,
    TriggerEvents::OneTime(ElementState::Pressed),
)];
