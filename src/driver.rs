use embedded_hal::i2c;

use crate::registers::*;
use crate::DriverError;
use crate::errors::map_i2c;
use crate::settings::{SettingsState, Settings, Sensitivity, SENSITIVITY_FACTORS};

pub struct GyroscopeDriver<I2cBus> {
    i2c: I2cBus,
    settings: SettingsState,
}
#[derive(Copy, Clone, Debug)]
pub struct AngularRates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<I2cBus: i2c::I2c> GyroscopeDriver<I2cBus> {
    pub fn new(i2c: I2cBus) -> Self {
        Self {i2c, settings: SettingsState::Unloaded}
    }
    pub fn read_relative_temp(&mut self) -> Result<i8, DriverError> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(SADR, &[OUT_TEMP], &mut buf).map_err(map_i2c)?;
        Ok(-(buf[0] as i8))
    }
    pub fn load_settings(&mut self) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(SADR, &[CTRL_REG1], &mut buf).map_err(map_i2c)?;
        if buf[0] != 0x0f {
            return Err(DriverError::SettingsError("CTRL_REG1 should be set to default"));
        }

        self.i2c.write_read(SADR, &[CTRL_REG2], &mut buf).map_err(map_i2c)?;
        if buf[0] != 0x00 {
            return Err(DriverError::SettingsError("CTRL_REG2 should be set to default"));
        }

        self.i2c.write_read(SADR, &[CTRL_REG3], &mut buf).map_err(map_i2c)?;
        if buf[0] != 0x00 {
            return Err(DriverError::SettingsError("CTRL_REG3 should be set to default"));
        }

        self.i2c.write_read(SADR, &[CTRL_REG4], &mut buf).map_err(map_i2c)?;
        let sensitivity_level = (buf[0] >> 4) & 0x03;
        let sensitivity = match sensitivity_level {
            0 => Sensitivity::Low,
            1 => Sensitivity::Medium,
            2 | 3 => Sensitivity::High,
            _ => return Err(DriverError::SettingsError("Invalid sensitivity level")),
        };

        self.i2c.write_read(SADR, &[CTRL_REG5], &mut buf).map_err(map_i2c)?;
        if buf[0] != 0x00 {
            return Err(DriverError::SettingsError("CTRL_REG5 should be set to default"));
        }

        self.settings = SettingsState::Loaded(Settings { sensitivity });

        Ok(())
    }
    pub fn read_angular_rates(&mut self) -> Result<AngularRates, DriverError> {
        if let SettingsState::Unloaded = self.settings {
            self.load_settings()?;
        }
        let scale = match &self.settings {
            SettingsState::Loaded(settings) => SENSITIVITY_FACTORS[settings.sensitivity as usize],
            SettingsState::Unloaded => return Err(DriverError::SettingsError("Settings not loaded")),
        };

        let mut buf = [0u8; 6];
        self.i2c.write_read(SADR, &[OUT_ANG_ALL], &mut buf).map_err(map_i2c)?;
        let x_int = (buf[1] as i16) << 8 | buf[0] as i16;
        let y_int = (buf[3] as i16) << 8 | buf[2] as i16;
        let z_int = (buf[5] as i16) << 8 | buf[4] as i16;

        Ok(AngularRates { x: x_int as f32 * scale, y: y_int as f32 * scale, z: z_int as f32 * scale })
    }
    pub fn set_sensitivity(&mut self, sensitivity: Sensitivity) -> Result<(), DriverError> {
        if let SettingsState::Unloaded = self.settings {
            self.load_settings()?;
        }
        let mut buf = [0u8; 1];
        self.i2c.write_read(SADR, &[CTRL_REG4], &mut buf).map_err(map_i2c)?;
        let sensitivity_level = match sensitivity {
            Sensitivity::Low => 0,
            Sensitivity::Medium => 1,
            Sensitivity::High => 2,
        };
        buf[0] = buf[0] & 0b11001111 | sensitivity_level << 4;
        self.i2c.write(SADR, &[CTRL_REG4, buf[0]]).map_err(map_i2c)?;
        self.settings = SettingsState::Loaded(Settings { sensitivity });
        Ok(())
    }
}
