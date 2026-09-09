pub(crate) const SENSITIVITY_FACTORS: [f32; 3] = [0.00875, 0.0175, 0.07];

#[derive(Copy, Clone)]
pub enum Sensitivity {
    Dps250 = 0,
    Dps500 = 1,
    Dps2000 = 2,
}

#[derive(Copy, Clone)]
pub enum OutputDataRate {
    Hz100 = 0,
    Hz200 = 1,
    Hz400 = 2,
    Hz800 = 3,
}

#[derive(Copy, Clone)]
pub enum Bandwidth {
    Low = 0,
    Medium = 1,
    High = 2,
    Highest = 3,
}

#[derive(Copy, Clone)]
pub enum AxesEnabled {
    SLEEP = 0,
    X = 1,
    Y = 2,
    XY = 3,
    Z = 4,
    XZ = 5,
    YZ = 6,
    XYZ = 7,
}

#[derive(Copy, Clone)]
pub struct Settings {
    pub sensitivity: Sensitivity,
    pub data_rate: OutputDataRate,
    pub bandwidth: Bandwidth,
    pub axes_enabled: AxesEnabled,
}
