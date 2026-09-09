mod driver;
mod errors;
mod helpers;
mod registers;
mod settings;

pub use driver::{AngularRates, GyroscopeDriver};
pub use errors::DriverError;
pub use settings::{AxesEnabled, Bandwidth, OutputDataRate, Sensitivity, Settings};
