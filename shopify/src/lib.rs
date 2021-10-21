#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod result;
#[macro_use]
pub mod client;
mod types;

pub mod fulfillment_service;
pub mod inventory;
pub mod order;
pub mod pagination;
pub mod product;
pub mod shop;
pub mod variant;
