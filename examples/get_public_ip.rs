use std::env;

use anyhow::Result;
use tianyi_api::TianyiBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let password = env::var("TIANYI_PASSWORD").expect("TIANYI_PASSWORD environment variable not set");

    let tianyi_instance = TianyiBuilder::new()
        .username("useradmin")
        .password(&password)
        .build()
        .await?;

    let gw_info = tianyi_instance.gwinfo().await?;

    println!("Public IP: {}, IPv6: {}", gw_info.wan_ip, gw_info.wan_ipv6);

    Ok(())
}