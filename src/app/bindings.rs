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
    mods: ModifiersState,

    conditions: TriggerEvents,

    pub action: Actions,
}

impl<T: Eq> Binding<T> {
    const fn new(
        trigger: T,
        mods: ModifiersState,
        action: Actions,
        conditions: TriggerEvents,
    ) -> Self {
        Self {
            trigger,
            mods,
            action,
            conditions,
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

        &self.trigger == trigger && &self.mods == mods && condition
    }
}

pub const MOUSE_BINDINGS: &[Binding<MouseButton>] = &[
    Binding::new(
        MouseButton::Left,
        ModifiersState::empty(),
        Actions::SetDrawing,
        TriggerEvents::Toggle,
    ),
    Binding::new(
        MouseButton::Right,
        ModifiersState::CONTROL,
        Actions::ToggleDecorations,
        TriggerEvents::OneTime(ElementState::Pressed),
    ),
];

pub const KEYBOARD_BINDINGS: &[Binding<&'static str>] = &[Binding::new(
    "F",
    ModifiersState::empty(),
    Actions::ToggleFullscreen,
    TriggerEvents::OneTime(ElementState::Pressed),
)];
