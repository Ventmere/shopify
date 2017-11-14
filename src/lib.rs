extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate reqwest;
#[macro_use]
extern crate error_chain;

#[cfg(test)]
extern crate dotenv;


pub mod error;
#[macro_use]
pub mod client;
mod types;

pub mod shop;
pub mod order;
pub mod fulfillment_service;
