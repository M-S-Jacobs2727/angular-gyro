pub(crate) const SENSITIVITY_FACTORS: [f32; 3] = [0.00875, 0.0175, 0.07];

#[derive(Copy, Clone)]
pub enum Sensitivity {
    Dps250 = 0,
    Dps500 = 1,
    Dps2000 = 2,
}

#[derive(Copy, Clone)]
pub(crate) struct Settings {
    sensitivity: Sensitivity,
}

impl Settings {
    pub fn new() -> Self {
        Self { sensitivity: Sensitivity::Dps250 }
    }
    pub fn set_sensitivity(&mut self, sensitivity: Sensitivity) {
        self.sensitivity = sensitivity;
    }
    pub fn sensitivity(&self) -> Sensitivity {
        self.sensitivity
    }
}