use reqwest::Url;

use libmpax::api::DEFAULT_SERVER_URL;

pub fn build_url(route: &str) -> Url {
    Url::parse(format!("http://{}{}", DEFAULT_SERVER_URL, route).as_str()).unwrap()
}
