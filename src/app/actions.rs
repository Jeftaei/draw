#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actions {
    CloseWindow,
    Minimize,
    ToggleMaximize,
    ToggleDecorations,
    ToggleFullscreen,

    ToggleDrawMode,

    EnterDrawMode,
    ExitDrawMode,
    RedoDraw,
    UndoDraw,

    SetDrawing,
}

impl Actions {
    pub fn help(&self) -> &'static str {
        match self {
            Actions::CloseWindow => "Close window",
            Actions::Minimize => "Minimize window",
            Actions::ToggleMaximize => "Toggles maximize",
            Actions::ToggleDecorations => "Toggles decorations",
            Actions::ToggleFullscreen => "Toggles fullscreen",

            Actions::ToggleDrawMode => "Toggles draw mode",
            Actions::EnterDrawMode => "Enters draw mode",
            Actions::ExitDrawMode => "Exits draw mode",
            Actions::RedoDraw => "Redraws last undone action",
            Actions::UndoDraw => "Undoes last action",

            Actions::SetDrawing => "Starts drawing when cursor moved",
        }
    }
}
