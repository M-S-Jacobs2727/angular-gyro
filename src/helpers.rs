use crate::settings::{AxesEnabled, Bandwidth, OutputDataRate, Sensitivity};
use crate::registers::{MASK_AXES_ENABLED, MASK_ODR_BW, MASK_SENSITIVITY};

pub(crate) fn extract_odr_bw(byte: u8) -> (OutputDataRate, Bandwidth) {
    let filtered = byte >> 4;
    match filtered {
        0 => (OutputDataRate::Hz100, Bandwidth::Low),
        1 => (OutputDataRate::Hz100, Bandwidth::Medium),
        2 => (OutputDataRate::Hz100, Bandwidth::High),
        3 => (OutputDataRate::Hz100, Bandwidth::Highest),
        4 => (OutputDataRate::Hz200, Bandwidth::Low),
        5 => (OutputDataRate::Hz200, Bandwidth::Medium),
        6 => (OutputDataRate::Hz200, Bandwidth::High),
        7 => (OutputDataRate::Hz200, Bandwidth::Highest),
        8 => (OutputDataRate::Hz400, Bandwidth::Low),
        9 => (OutputDataRate::Hz400, Bandwidth::Medium),
        10 => (OutputDataRate::Hz400, Bandwidth::High),
        11 => (OutputDataRate::Hz400, Bandwidth::Highest),
        12 => (OutputDataRate::Hz800, Bandwidth::Low),
        13 => (OutputDataRate::Hz800, Bandwidth::Medium),
        14 => (OutputDataRate::Hz800, Bandwidth::High),
        15 => (OutputDataRate::Hz800, Bandwidth::Highest),
        _ => panic!("Impossible byte passed to extract_odr_bw!")
    }
}

pub(crate) fn apply_odr_bw(byte: u8, data_rate: OutputDataRate, bandwidth: Bandwidth) -> u8 {
    let masked_byte = match (data_rate, bandwidth) {
        (OutputDataRate::Hz100, Bandwidth::Low) => 0,
        (OutputDataRate::Hz100, Bandwidth::Medium) => 1,
        (OutputDataRate::Hz100, Bandwidth::High) => 2,
        (OutputDataRate::Hz100, Bandwidth::Highest) => 3,
        (OutputDataRate::Hz200, Bandwidth::Low) => 4,
        (OutputDataRate::Hz200, Bandwidth::Medium) => 5,
        (OutputDataRate::Hz200, Bandwidth::High) => 6,
        (OutputDataRate::Hz200, Bandwidth::Highest) => 7,
        (OutputDataRate::Hz400, Bandwidth::Low) => 8,
        (OutputDataRate::Hz400, Bandwidth::Medium) => 9,
        (OutputDataRate::Hz400, Bandwidth::High) => 10,
        (OutputDataRate::Hz400, Bandwidth::Highest) => 11,
        (OutputDataRate::Hz800, Bandwidth::Low) => 12,
        (OutputDataRate::Hz800, Bandwidth::Medium) => 13,
        (OutputDataRate::Hz800, Bandwidth::High) => 14,
        (OutputDataRate::Hz800, Bandwidth::Highest) => 15,
    };
    masked_byte | (byte & !MASK_ODR_BW)
}

pub(crate) fn extract_axes_enabled(byte: u8) -> AxesEnabled {
    let filtered = byte & MASK_AXES_ENABLED;
    match filtered {
        0 => AxesEnabled::SLEEP,
        1 => AxesEnabled::X,
        2 => AxesEnabled::Y,
        3 => AxesEnabled::XY,
        4 => AxesEnabled::Z,
        5 => AxesEnabled::XZ,
        6 => AxesEnabled::YZ,
        7 => AxesEnabled::XYZ,
        _ => panic!("Impossible byte passed to extract_axes_enabled!")
    }
}

pub(crate) fn apply_axes_enabled(byte: u8, axes_enabled: AxesEnabled) -> u8 {
    let masked_byte = match axes_enabled {
        AxesEnabled::SLEEP => 0,
        AxesEnabled::X => 1,
        AxesEnabled::Y => 2,
        AxesEnabled::XY => 3,
        AxesEnabled::Z => 4,
        AxesEnabled::XZ => 5,
        AxesEnabled::YZ => 6,
        AxesEnabled::XYZ => 7,
    };
    masked_byte | (byte & !MASK_AXES_ENABLED)
}

pub(crate) fn extract_sensitivity(byte: u8) -> Sensitivity {
    let filtered = (byte & MASK_SENSITIVITY) >> 4;
    match filtered {
        0 => Sensitivity::Dps250,
        1 => Sensitivity::Dps500,
        2 => Sensitivity::Dps2000,
        3 => Sensitivity::Dps2000,
        _ => panic!("Impossible byte passed to extract_sensitivity!")
    }
}

pub(crate) fn apply_sensitivity(byte: u8, sensitivity: Sensitivity) -> u8 {
    let filtered = byte & !MASK_SENSITIVITY;
    let sensitivity_level = match sensitivity {
        Sensitivity::Dps250 => 0,
        Sensitivity::Dps500 => 1,
        Sensitivity::Dps2000 => 2,
    };
    filtered | (sensitivity_level << 4)
}