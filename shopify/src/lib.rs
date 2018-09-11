extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(test)]
extern crate dotenv;

pub mod result;
#[macro_use]
pub mod client;
mod types;

pub mod fulfillment_service;
pub mod order;
pub mod product;
pub mod shop;
pub mod variant;
