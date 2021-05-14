use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::private::{add_cmd_key_mapping, CmdKeyMapping};

use self::super::ZapPropertyKey;

/// Zap Power on/off
pub struct Power;

impl typemap::Key for Power {
    type Value = bool;
}

impl crate::private::PropKeyInternal for Power {
    const CMD_KEY: &'static str = "ZAPONOFF";
}

impl ZapPropertyKey for Power {}

/// Zap parameters (query only)
pub struct Params;

impl typemap::Key for Params {
    type Value = ();
}

impl crate::private::PropKeyInternal for Params {
    const CMD_KEY: &'static str = "ZAPPARAMS";
}

impl ZapPropertyKey for Params {}

/// Zap digital voltage
pub struct DigitalVoltage;

impl typemap::Key for DigitalVoltage {
    type Value = f32;
}

impl crate::private::PropKeyInternal for DigitalVoltage {
    const CMD_KEY: &'static str = "ZAPDV";
}

impl ZapPropertyKey for DigitalVoltage {}

/// Zap Analog Voltage+
pub struct PositiveAnalogVoltage;

impl typemap::Key for PositiveAnalogVoltage {
    type Value = f32;
}

impl crate::private::PropKeyInternal for PositiveAnalogVoltage {
    const CMD_KEY: &'static str = "ZAPAPV";
}

impl ZapPropertyKey for PositiveAnalogVoltage {}

/// Zap Analog Voltage-
pub struct NegativeAnalogVoltage;

impl typemap::Key for NegativeAnalogVoltage {
    type Value = f32;
}

impl crate::private::PropKeyInternal for NegativeAnalogVoltage {
    const CMD_KEY: &'static str = "ZAPANV";
}

impl ZapPropertyKey for NegativeAnalogVoltage {}

lazy_static! {
pub(crate) static ref CMD_KEY_MAPPING: CmdKeyMapping = {
    let mut hash_map = HashMap::new();
    add_cmd_key_mapping(&mut hash_map, Power);
    add_cmd_key_mapping(&mut hash_map, Params);
    add_cmd_key_mapping(&mut hash_map, DigitalVoltage);
    add_cmd_key_mapping(&mut hash_map, PositiveAnalogVoltage);
    add_cmd_key_mapping(&mut hash_map, NegativeAnalogVoltage);
    hash_map
};
}

