#![feature(plugin)]
#![plugin(interpolate_idents)]
extern crate top_sdk;
#[macro_use]
extern crate quick_error;

pub mod requests;
mod error;
mod client;
pub use error::XiamiError;
pub use client::XiamiClient;
