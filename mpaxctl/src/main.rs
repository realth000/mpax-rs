use std::net::Ipv4Addr;

use reqwest::Client;
use reqwest::redirect::Policy;

use libmpax::add;

mod config;

/// Build a http client instance.
fn build_net_client() -> Client {
    return Client::builder()
        .redirect(Policy::none())
        .local_address(Ipv4Addr::new(127, 0, 0, 1).into())
        .build()
        .unwrap();
}

fn main() {
    let x = add(1, 2);
    println!("Hello, world! {x}");
}
