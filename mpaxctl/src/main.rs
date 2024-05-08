use reqwest::Client;
use reqwest::redirect::Policy;

use libmpax::add;

mod config;
mod cmd;

/// Build a http client instance.
fn build_net_client() -> Client {
    return Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap();
}

fn main() {
    let x = add(1, 2);
    println!("Hello, world! {x}");
    let client = build_net_client();
}
