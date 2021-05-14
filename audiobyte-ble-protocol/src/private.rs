use std::collections::HashMap;

use crate::{PropertyKeyDelegate, PropertyKey};

pub const CMD_PREFIX: &'static str = "\\";

pub trait PropKeyInternal: typemap::Key
    where <Self as typemap::Key>::Value: Clone {
    const CMD_KEY: &'static str;
}

pub fn add_cmd_key_mapping<K>(hash_map: &mut HashMap<&'static str, Box<dyn PropertyKeyDelegate>>, k: K)
    where K: PropertyKey + PropertyKeyDelegate,
          <K as typemap::Key>::Value: Clone {
    let cmd_key = K::CMD_KEY;
    hash_map.insert(cmd_key, Box::new(k));
}

pub type CmdKeyMapping = HashMap<&'static str, Box<dyn PropertyKeyDelegate>>;
