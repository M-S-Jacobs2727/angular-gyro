use embedded_hal::i2c;

use crate::registers::*;
use crate::DriverError;
use crate::errors::map_i2c;
use crate::settings::{Settings, Sensitivity, SENSITIVITY_FACTORS};

pub struct GyroscopeDriver<I2cBus> {
    i2c: I2cBus,
    settings: Settings,
}
#[derive(Copy, Clone, Debug)]
pub struct AngularRates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<I2cBus: i2c::I2c> GyroscopeDriver<I2cBus> {
    pub fn new(i2c: I2cBus) -> Result<Self, DriverError> {
        let mut driver = Self {i2c, settings: Settings::new()};
        driver.load_settings()?;
        Ok(driver)
    }
    pub fn read_relative_temp(&mut self) -> Result<i8, DriverError> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(SADR, &[OUT_TEMP], &mut buf).map_err(map_i2c)?;
        Ok(buf[0] as i8)
    }
    fn load_settings(&mut self) -> Result<(), DriverError> {
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
        let sensitivity = self.read_sensitivity()?;

        self.i2c.write_read(SADR, &[CTRL_REG5], &mut buf).map_err(map_i2c)?;
        if buf[0] != 0x00 {
            return Err(DriverError::SettingsError("CTRL_REG5 should be set to default"));
        }

        self.settings.set_sensitivity(sensitivity);
        Ok(())
    }
    pub fn read_angular_rates(&mut self) -> Result<AngularRates, DriverError> {
        let scale = SENSITIVITY_FACTORS[self.settings.sensitivity() as usize];

        let mut buf = [0u8; 6];
        self.i2c.write_read(SADR, &[OUT_ANG_ALL], &mut buf).map_err(map_i2c)?;
        let x_int = i16::from_le_bytes([buf[0], buf[1]]);
        let y_int = i16::from_le_bytes([buf[2], buf[3]]);
        let z_int = i16::from_le_bytes([buf[4], buf[5]]);

        Ok(AngularRates { x: x_int as f32 * scale, y: y_int as f32 * scale, z: z_int as f32 * scale })
    }
    pub fn set_sensitivity(&mut self, sensitivity: Sensitivity) -> Result<(), DriverError> {
        self.write_sensitivity(sensitivity)?;
        self.settings.set_sensitivity(sensitivity);
        
        Ok(())
    }

    fn read_sensitivity(&mut self) -> Result<Sensitivity, DriverError> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(SADR, &[CTRL_REG4], &mut buf).map_err(map_i2c)?;
        let sensitivity_level = (buf[0] & MASK_SENSITIVITY) >> 4;
        let sensitivity = match sensitivity_level {
            0 => Sensitivity::Dps250,
            1 => Sensitivity::Dps500,
            2 | 3 => Sensitivity::Dps2000,
            _ => return Err(DriverError::SettingsError("Invalid sensitivity level")),
        };
        Ok(sensitivity)
    }
    fn write_sensitivity(&mut self, sensitivity: Sensitivity) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(SADR, &[CTRL_REG4], &mut buf).map_err(map_i2c)?;
        let sensitivity_level = match sensitivity {
            Sensitivity::Dps250 => 0,
            Sensitivity::Dps500 => 1,
            Sensitivity::Dps2000 => 2,
        };
        buf[0] = buf[0] & !MASK_SENSITIVITY | sensitivity_level << 4;
        self.i2c.write(SADR, &[CTRL_REG4, buf[0]]).map_err(map_i2c)?;
        Ok(())
    }
}
