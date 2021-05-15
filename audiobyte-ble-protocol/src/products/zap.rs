use std::borrow::Cow;

use crate::{ProductType, Result};
use crate::private::{CmdKeyMapping, ProductTypeInternal, PropKeyInternal};

pub mod keys;

pub trait ZapPropertyKey {
    const CHARACTERISTIC: u16 = 0xFFE1;
}

impl<K> crate::PropertyKey for K
    where K: ZapPropertyKey + PropKeyInternal,
          <K as typemap::Key>::Value: Clone {
    const CHARACTERISTIC: u16 = <Self as ZapPropertyKey>::CHARACTERISTIC;

    fn parse_value(line: &str) -> Result<Cow<<Self as typemap::Key>::Value>> {
        todo!()
    }
}

pub struct Zap {}

impl ProductTypeInternal for Zap {
    fn cmd_key_mapping() -> &'static CmdKeyMapping {
        &keys::CMD_KEY_MAPPING
    }
}

impl ProductType for Zap {
}

