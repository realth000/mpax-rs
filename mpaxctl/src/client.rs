use reqwest::redirect::Policy;
use reqwest::Client;

/// Build a http client instance.
#[allow(clippy::module_name_repetitions)]
pub fn build_net_client() -> Client {
    Client::builder().redirect(Policy::none()).build().unwrap()
}
