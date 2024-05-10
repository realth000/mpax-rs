use reqwest::redirect::Policy;
use reqwest::Client;

/// Build a http client instance.
pub fn build_net_client() -> Client {
    return Client::builder().redirect(Policy::none()).build().unwrap();
}
