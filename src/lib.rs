extern crate top_sdk;
use top_sdk::*;

mod requests;

pub struct XiamiClient {
    top_client: TaobaoClient,
}