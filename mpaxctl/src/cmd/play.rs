use anyhow::{bail, Result};
use reqwest::{StatusCode, Url};

use crate::client::build_net_client;
use crate::cmd::PlayArgs;
use crate::config::DEFAULT_SERVER_URL;

pub async fn handle_play_command(args: PlayArgs) -> Result<()> {
    let play_target = args.play_target;
    println!("??>?? {play_target:#?}");
    let next = play_target.next.unwrap();
    let prev = play_target.prev.unwrap();
    if next || prev {
        bail!("unsupported play format");
    }

    let file_path = play_target.file.to_owned().unwrap();

    println!(">>> {:#?}", play_target);

    let mut url = Url::parse(DEFAULT_SERVER_URL).unwrap();
    let mut query_pairs = url.query_pairs_mut();
    query_pairs.append_pair("filePath", file_path.as_str());
    query_pairs.finish();
    drop(query_pairs);
    let client = build_net_client();

    let resp = client.get(url).send().await?;
    if resp.status() != StatusCode::OK {
        bail!("server replied error: code={:#?}", resp.status());
    }
    Ok(())
}
