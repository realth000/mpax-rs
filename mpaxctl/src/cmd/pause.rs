use anyhow::{bail, Result};
use log::{debug, info};
use reqwest::StatusCode;

use libmpax::api::ROUTE_ACTION_PAUSE;

use crate::client::build_net_client;
use crate::cmd::PauseArgs;
use crate::url::build_url;

pub async fn handle_pause_command(args: PauseArgs) -> Result<()> {
    let url = build_url(ROUTE_ACTION_PAUSE);
    debug!("{} run pause command", url);
    let client = build_net_client();
    let resp = client.get(url).send().await?;
    if resp.status() != StatusCode::OK {
        bail!("server replied error: code={:#?}", resp.status())
    }
    Ok(())
}
