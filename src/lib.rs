#![feature(plugin)]
#![plugin(interpolate_idents)]
extern crate top_sdk;
use top_sdk::*;

pub mod requests;

pub struct XiamiClient {
    top_client: TaobaoClient,
}
