use std::any::Any;
use std::collections::HashMap;

use crate::{PropertyKey, PropertyKeyDelegate, PropertyKeyTypeId};
use crate::Error::InvalidKey;
use crate::Result;

pub const CMD_PREFIX: &'static str = "\\";

pub trait PropKeyInternal: typemap::Key
    where <Self as typemap::Key>::Value: Clone {
    const CMD_KEY: &'static str;
}

pub trait ProductTypeInternal {
    fn cmd_key_mapping() -> &'static CmdKeyMapping;

    fn parse_property(mut line: &str) -> Result<(PropertyKeyTypeId, Box<dyn Any>)> {
        if line.starts_with(CMD_PREFIX) {
            line = &line[CMD_PREFIX.len()..];
        }
        for (cmd_key, prop_key) in Self::cmd_key_mapping().iter() {
            if line.starts_with(cmd_key) {
                return Ok((prop_key.key_type_id(), prop_key.parse_value(line)?));
            }
        }
        return Err(InvalidKey);
    }
}

pub fn add_cmd_key_mapping<K>(hash_map: &mut HashMap<&'static str, Box<dyn PropertyKeyDelegate>>, k: K)
    where K: PropertyKey + PropertyKeyDelegate,
          <K as typemap::Key>::Value: Clone {
    let cmd_key = K::CMD_KEY;
    hash_map.insert(cmd_key, Box::new(k));
}

pub type CmdKeyMapping = HashMap<&'static str, Box<dyn PropertyKeyDelegate>>;
