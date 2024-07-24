use stripe::blocking::Client;

use crate::get_base_test_config;

mod account;
mod charge;
mod checkout;
mod customer;
mod invoice;
mod plan_interval;
mod price;
mod product;
mod promotion_code;
mod subscription;
mod subscription_item;
mod token;
mod transfer_reversal;

pub fn get_client() -> Client {
    get_base_test_config().build_sync().expect("could not build client")
}
