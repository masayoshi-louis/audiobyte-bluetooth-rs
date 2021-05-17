use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::private::{add_cmd_key_mapping, CmdKeyMapping};

use self::super::HydraZapPropertyKey;

/// Zap power on/off
pub struct Power;

impl typemap::Key for Power {
    type Value = bool;
}

impl crate::private::PropKeyInternal for Power {
    const CMD_KEY: &'static str = "ZAPONOFF";
}

impl HydraZapPropertyKey for Power {}

/// Zap parameters (query only)
pub struct Params;

impl typemap::Key for Params {
    type Value = ();
}

impl crate::private::PropKeyInternal for Params {
    const CMD_KEY: &'static str = "ZAPPARAMS";
}

impl HydraZapPropertyKey for Params {}

/// Zap digital voltage
pub struct DigitalVoltage;

impl typemap::Key for DigitalVoltage {
    type Value = f32;
}

impl crate::private::PropKeyInternal for DigitalVoltage {
    const CMD_KEY: &'static str = "ZAPDV";
}

impl HydraZapPropertyKey for DigitalVoltage {}

/// Zap analog voltage+
pub struct PositiveAnalogVoltage;

impl typemap::Key for PositiveAnalogVoltage {
    type Value = f32;
}

impl crate::private::PropKeyInternal for PositiveAnalogVoltage {
    const CMD_KEY: &'static str = "ZAPAPV";
}

impl HydraZapPropertyKey for PositiveAnalogVoltage {}

/// Zap analog voltage-
pub struct NegativeAnalogVoltage;

impl typemap::Key for NegativeAnalogVoltage {
    type Value = f32;
}

impl crate::private::PropKeyInternal for NegativeAnalogVoltage {
    const CMD_KEY: &'static str = "ZAPANV";
}

impl HydraZapPropertyKey for NegativeAnalogVoltage {}

/// Zap digital capacitor SP(?)
pub struct DigitalCapacitorSP;

impl typemap::Key for DigitalCapacitorSP {
    type Value = i32;
}

impl crate::private::PropKeyInternal for DigitalCapacitorSP {
    const CMD_KEY: &'static str = "ZAPDCAPSP";
}

impl HydraZapPropertyKey for DigitalCapacitorSP {}

/// Zap digital power on/off
pub struct DigitalPower;

impl typemap::Key for DigitalPower {
    type Value = bool;
}

impl crate::private::PropKeyInternal for DigitalPower {
    const CMD_KEY: &'static str = "ZAPD";
}

impl HydraZapPropertyKey for DigitalPower {}

/// Zap analog power on/off
pub struct AnalogPower;

impl typemap::Key for AnalogPower {
    type Value = bool;
}

impl crate::private::PropKeyInternal for AnalogPower {
    const CMD_KEY: &'static str = "ZAPA";
}

impl HydraZapPropertyKey for AnalogPower {}

/// Zap reference power on/off
pub struct ReferencePower;

impl typemap::Key for ReferencePower {
    type Value = bool;
}

impl crate::private::PropKeyInternal for ReferencePower {
    const CMD_KEY: &'static str = "ZAPR";
}

impl HydraZapPropertyKey for ReferencePower {}

// TODO move
// /// Vox connections
// pub struct VoxConnection;
//
// impl typemap::Key for VoxConnection {
//     type Value = bool;
// }
//
// impl crate::private::PropKeyInternal for VoxConnection {
//     const CMD_KEY: &'static str = "VOXCONN";
// }
//
// impl ZapPropertyKey for VoxConnection {}
//
// /// Hub connections
// pub struct HubConnection;
//
// impl typemap::Key for HubConnection {
//     type Value = bool;
// }
//
// impl crate::private::PropKeyInternal for HubConnection {
//     const CMD_KEY: &'static str = "HUBCONN";
// }
//
// impl ZapPropertyKey for HubConnection {}

lazy_static! {
pub(super) static ref CMD_KEY_MAPPING: CmdKeyMapping = {
    let mut hash_map = HashMap::new();
    add_cmd_key_mapping(&mut hash_map, Power);
    add_cmd_key_mapping(&mut hash_map, Params);
    add_cmd_key_mapping(&mut hash_map, DigitalVoltage);
    add_cmd_key_mapping(&mut hash_map, PositiveAnalogVoltage);
    add_cmd_key_mapping(&mut hash_map, NegativeAnalogVoltage);
    add_cmd_key_mapping(&mut hash_map, DigitalCapacitorSP);
    add_cmd_key_mapping(&mut hash_map, DigitalPower);
    add_cmd_key_mapping(&mut hash_map, AnalogPower);
    add_cmd_key_mapping(&mut hash_map, ReferencePower);
    hash_map
};
}
