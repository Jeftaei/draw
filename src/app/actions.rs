#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actions {
    CloseWindow,
    Minimize,
    ToggleMaximize,
    ToggleDecorations,
    ToggleFullscreen,

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

            Actions::SetDrawing => "Starts drawing when cursor moved",
        }
    }
}
