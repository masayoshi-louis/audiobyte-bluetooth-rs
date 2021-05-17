use audiobyte_ble_protocol::products;
use audiobyte_ble_protocol::ProductType;

pub trait Device {
    type ProductType: ProductType;

    fn get(&self);
}

pub struct HydraZap {}

impl Device for HydraZap {
    type ProductType = products::zap::HydraZap;
}