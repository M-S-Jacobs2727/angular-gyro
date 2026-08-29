use embedded_hal::i2c;

pub(crate) fn map_i2c<E: i2c::Error>(e: E) -> DriverError {
    DriverError::I2cError(e.kind())
}

#[derive(Debug)]
pub enum DriverError {
    I2cError(i2c::ErrorKind),
    SettingsError(&'static str),
}
impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for DriverError {}
