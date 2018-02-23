extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(test)]
extern crate dotenv;

pub mod error;
#[macro_use]
pub mod client;
mod types;

pub mod shop;
pub mod order;
pub mod fulfillment_service;
pub mod product;
