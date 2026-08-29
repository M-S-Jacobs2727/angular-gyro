pub(crate) const SADR: u8 = 0x68;
pub(crate) const CTRL_REG1: u8 = 0x20; // default: 0b00000111
pub(crate) const CTRL_REG2: u8 = 0x21; // default: 0b00000000
pub(crate) const CTRL_REG3: u8 = 0x22; // default: 0b00000000
pub(crate) const CTRL_REG4: u8 = 0x23; // default: 0b00000000
pub(crate) const CTRL_REG5: u8 = 0x24; // default: 0b00000000
pub(crate) const REFERENCE: u8 = 0x25; // default: 0b00000000
pub(crate) const OUT_TEMP: u8 =  0x26;
pub(crate) const OUT_X_L: u8 = 0x28;
pub(crate) const OUT_X_H: u8 = 0x29;
pub(crate) const OUT_Y_L: u8 = 0x2a;
pub(crate) const OUT_Y_H: u8 = 0x2b;
pub(crate) const OUT_Z_L: u8 = 0x2c;
pub(crate) const OUT_Z_H: u8 = 0x2d;
pub(crate) const OUT_ANG_ALL: u8 = 0xa8; // == 0x28 | 0x80 (MSB indicates continuous read mode)