pub(crate) const SENSITIVITY_FACTORS: [f32; 3] = [0.00875, 0.0175, 0.07];

#[derive(Copy, Clone)]
pub enum Sensitivity {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[derive(Copy, Clone)]
pub(crate) enum SettingsState {
    Unloaded,
    Loaded(Settings),
}

#[derive(Copy, Clone)]
pub(crate) struct Settings {
    pub sensitivity: Sensitivity,
}