use std::borrow::Cow;

use crate::private::PropKeyInternal;
use crate::Result;

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
