use embedded_hal::i2c;

use crate::DriverError;
use crate::errors::map_i2c;
use crate::helpers::{
    apply_axes_enabled, apply_odr_bw, apply_sensitivity, extract_axes_enabled, extract_odr_bw,
    extract_sensitivity,
};
use crate::registers::*;
use crate::settings::{
    AxesEnabled, Bandwidth, OutputDataRate, SENSITIVITY_FACTORS, Sensitivity, Settings,
};

pub struct GyroscopeDriver<I2cBus> {
    i2c: I2cBus,
}
#[derive(Copy, Clone, Debug)]
pub struct AngularRates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<I2cBus: i2c::I2c> GyroscopeDriver<I2cBus> {
    pub fn new(i2c: I2cBus) -> Result<Self, DriverError> {
        Ok(Self { i2c })
    }
    fn read_register(&mut self, register: u8, buffer: &mut [u8]) -> Result<(), DriverError> {
        self.i2c
            .write_read(SADR, &[register], buffer)
            .map_err(map_i2c)?;
        Ok(())
    }
    fn write_register(&mut self, register: u8, buffer: &[u8]) -> Result<(), DriverError> {
        let mut new_buffer = [0u8; 16];
        let total = buffer.len() + 1;
        if total > new_buffer.len() {
            return Err(DriverError::SettingsError("write buffer too large"));
        }
        new_buffer[0] = register;
        new_buffer[1..total].copy_from_slice(buffer);
        self.i2c
            .write(SADR, &new_buffer[..total])
            .map_err(map_i2c)?;
        Ok(())
    }
    pub fn power_up(&mut self) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        buf[0] |= 0x08;
        self.write_register(CTRL_REG1, &buf)?;
        Ok(())
    }
    pub fn power_down(&mut self) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        buf[0] &= !0x08;
        self.write_register(CTRL_REG1, &buf)?;
        Ok(())
    }
    pub fn read_relative_temp(&mut self) -> Result<i8, DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(OUT_TEMP, &mut buf)?;
        Ok(buf[0] as i8)
    }
    pub fn read_angular_rates(&mut self) -> Result<AngularRates, DriverError> {
        let sensitivity = self.read_sensitivity()?;
        let scale = SENSITIVITY_FACTORS[sensitivity as usize];

        let mut buf = [0u8; 6];
        self.read_register(OUT_ANG_ALL, &mut buf)?;
        let x_int = i16::from_le_bytes([buf[0], buf[1]]);
        let y_int = i16::from_le_bytes([buf[2], buf[3]]);
        let z_int = i16::from_le_bytes([buf[4], buf[5]]);

        Ok(AngularRates {
            x: x_int as f32 * scale,
            y: y_int as f32 * scale,
            z: z_int as f32 * scale,
        })
    }

    pub fn read_settings(&mut self) -> Result<Settings, DriverError> {
        let mut buf = [0u8; 5];
        self.read_register(CTRL_REG_ALL, &mut buf)?;
        let (data_rate, bandwidth) = extract_odr_bw(buf[0]);
        let axes_enabled = extract_axes_enabled(buf[0]);
        let sensitivity = extract_sensitivity(buf[3]);

        Ok(Settings {
            sensitivity,
            data_rate,
            bandwidth,
            axes_enabled,
        })
    }
    pub fn read_sensitivity(&mut self) -> Result<Sensitivity, DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG4, &mut buf)?;
        Ok(extract_sensitivity(buf[0]))
    }
    pub fn write_sensitivity(&mut self, sensitivity: Sensitivity) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG4, &mut buf)?;
        buf[0] = apply_sensitivity(buf[0], sensitivity);
        self.write_register(CTRL_REG4, &buf)?;
        Ok(())
    }
    pub fn read_data_rate(&mut self) -> Result<OutputDataRate, DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        Ok(extract_odr_bw(buf[0]).0)
    }
    pub fn read_bandwidth(&mut self) -> Result<Bandwidth, DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        Ok(extract_odr_bw(buf[0]).1)
    }
    pub fn read_odr_bw(&mut self) -> Result<(OutputDataRate, Bandwidth), DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        Ok(extract_odr_bw(buf[0]))
    }
    pub fn read_axes_enabled(&mut self) -> Result<AxesEnabled, DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        Ok(extract_axes_enabled(buf[0]))
    }
    pub fn write_data_rate(&mut self, data_rate: OutputDataRate) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        let (_, bandwidth) = extract_odr_bw(buf[0]);
        buf[0] = apply_odr_bw(buf[0], data_rate, bandwidth);
        self.write_register(CTRL_REG1, &buf)?;
        Ok(())
    }
    pub fn write_bandwidth(&mut self, bandwidth: Bandwidth) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        let (data_rate, _) = extract_odr_bw(buf[0]);
        buf[0] = apply_odr_bw(buf[0], data_rate, bandwidth);
        self.write_register(CTRL_REG1, &buf)?;
        Ok(())
    }
    pub fn write_odr_bw(
        &mut self,
        data_rate: OutputDataRate,
        bandwidth: Bandwidth,
    ) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        buf[0] = apply_odr_bw(buf[0], data_rate, bandwidth);
        self.write_register(CTRL_REG1, &buf)?;
        Ok(())
    }
    pub fn write_axes_enabled(&mut self, axes_enabled: AxesEnabled) -> Result<(), DriverError> {
        let mut buf = [0u8; 1];
        self.read_register(CTRL_REG1, &mut buf)?;
        buf[0] = apply_axes_enabled(buf[0], axes_enabled);
        self.write_register(CTRL_REG1, &buf)?;
        Ok(())
    }
    pub fn sleep(&mut self) -> Result<(), DriverError> {
        self.write_axes_enabled(AxesEnabled::SLEEP)
    }
}
