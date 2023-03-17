use anyhow::Result;
use tianyi_api::TianyiBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let tianyi_instance = TianyiBuilder::new()
        .username("useradmin")
        .password("***")
        .build()
        .await?;

    let old_ip = "192.168.1.11";
    let new_ip = "192.168.1.12";

    tianyi_instance.update_port_forwarding_rule(old_ip, new_ip).await?;

    println!("Port forwarding rules updated successfully.");

    Ok(())
}