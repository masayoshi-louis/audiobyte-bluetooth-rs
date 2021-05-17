use std::borrow::Cow;

use crate::{ProductType, Result};
use crate::private::{CmdKeyMapping, ProductTypeInternal, PropKeyInternal};

pub mod keys;

const ZAP_CHARACTERISTIC: u16 = 0xFFE1;

pub trait HydraZapPropertyKey {}

impl<K> crate::PropertyKey for K
    where K: HydraZapPropertyKey + PropKeyInternal,
          <K as typemap::Key>::Value: Clone {
    const CHARACTERISTIC: u16 = ZAP_CHARACTERISTIC;

    fn parse_value(line: &str) -> Result<Cow<<Self as typemap::Key>::Value>> {
        todo!()
    }
}

pub struct HydraZap {}

impl ProductTypeInternal for HydraZap {
    fn cmd_key_mapping() -> &'static CmdKeyMapping {
        &keys::CMD_KEY_MAPPING
    }
}

impl ProductType for HydraZap { type PropertyKey = dyn HydraZapPropertyKey; }

