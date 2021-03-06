use std::any::{Any, TypeId};
use std::borrow::Cow;

use crate::private::{ProductTypeInternal, PropKeyInternal};

pub mod products;

mod private;

pub enum Error {
    InvalidKey = 1,
    InvalidValue = 2,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait PropertyKey: PropKeyInternal
    where <Self as typemap::Key>::Value: Clone {
    const CHARACTERISTIC: u16;

    fn key_type_id() -> PropertyKeyTypeId {
        return PropertyKeyTypeId(TypeId::of::<Self>());
    }

    fn parse_value(line: &str) -> Result<Cow<<Self as typemap::Key>::Value>>;
}

pub trait PropertyKeyDelegate: Send + Sync {
    fn key_type_id(&self) -> PropertyKeyTypeId;

    fn characteristic(&self) -> u16;

    fn parse_value(&self, line: &str) -> Result<Box<dyn Any>>;
}

impl<K> PropertyKeyDelegate for K
    where K: PropertyKey,
          K: Send + Sync,
          <K as typemap::Key>::Value: Clone,
          <K as typemap::Key>::Value: Any + 'static {
    fn key_type_id(&self) -> PropertyKeyTypeId {
        <Self as PropertyKey>::key_type_id()
    }

    fn characteristic(&self) -> u16 {
        Self::CHARACTERISTIC
    }

    fn parse_value(&self, line: &str) -> Result<Box<dyn Any>> {
        let cow = Self::parse_value(line)?;
        return Ok(Box::new(cow.into_owned()));
    }
}

pub trait ProductType: ProductTypeInternal {
    fn parse_property(line: &str) -> Result<(PropertyKeyTypeId, Box<dyn Any>)> {
        <Self as ProductTypeInternal>::parse_property(line)
    }
}

pub struct PropertyKeyTypeId(TypeId);