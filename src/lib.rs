//! # Tianyi API
//!
//! `tianyi_api` is an unofficial API implementation for the China Telecom Tianyi router.
//! It allows you to perform operations such as obtaining public IP addresses, updating
//! port forwarding rules, and more.
//!
//! This library provides an asynchronous API built with `tokio` and error handling
//! with `anyhow`. It is designed to be simple to use and efficient.
//!
//! ## Features
//!
//! - Get public IP address
//! - Manage port forwarding rules (add, delete, enable, disable)
//! - Retrieve gateway information
//! - Logout from the router
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tokio = { version = "1.26", features = ["full"] }
//! tianyi_api = { git = "https://github.com/edsky/tianyi_api.git" }
//! ```
//!
//! Then, in your application, you can use the `Tianyi` struct to interact with the router:
//!
//! ```rust
//! use tianyi_api::TianyiBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!     let tianyi = TianyiBuilder::new()
//!         .username("useradmin")
//!         .password("password")
//!         .build()
//!         .await?;
//!
//!     let public_ip = tianyi.wanip().await?;
//!     println!("Public IP: {}", public_ip);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! You can find examples for each operation in the [examples](https://github.com/edsky/tianyi_api/tree/main/examples) directory of the GitHub repository.
//!
//! ## Contributing
//!
//! Contributions are welcome! Please feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/edsky/tianyi_api).
//!
//! ## License
//!
//! This project is licensed under the [MIT License](https://github.com/edsky/tianyi_api/blob/main/LICENSE).
//!
//! ## Disclaimer
//!
//! This library is not affiliated with, endorsed by, or supported by China Telecom or any other official entity.
//! Use of this library is at your own risk. The authors and contributors are not responsible for any damage or issues that may arise from using this library.
//!
use std::collections::HashMap;

use anyhow::{Context, Result};
use reqwest::{Client, Proxy};
use serde::Deserialize;
use rand::Rng;

const DEFAULT_IP: &str = "192.168.1.1";
const DEFAULT_UNAME: &str = "useradmin";
const DEFAULT_UPWD: &str = "";

/// `TianyiBuilder` is a builder for the `Tianyi` struct.
///
/// This builder allows you to set the router's IP address, username, and password before creating a `Tianyi` instance.
pub struct TianyiBuilder {
    ip: String,
    username: String,
    password: String,
}

impl Default for TianyiBuilder {
    fn default() -> Self {
        Self {
            ip: DEFAULT_IP.to_string(),
            username: DEFAULT_UNAME.to_string(),
            password: DEFAULT_UPWD.to_string(),
        }
    }
}

impl TianyiBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ip(mut self, ip: &str) -> Self {
        self.ip = ip.to_string();
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    pub async fn build(self) -> Result<Tianyi> {
        Tianyi::new(&self.ip, &self.username, &self.password).await
    }
}

/// The `Tianyi` struct represents a connection to a Tianyi router and provides methods to interact with it.
pub struct Tianyi {
    url: String,
    token: String,
    client: Client,
}

/// Represents the gateway information returned by the router.
#[derive(Debug, Deserialize)]
pub struct GatewayInfo {
    #[serde(rename = "LANIP")]
    lan_ip: String,
    #[serde(rename = "LANIPv6")]
    lan_ipv6: String,
    #[serde(rename = "MAC")]
    mac: String,
    #[serde(rename = "WANIP")]
    wan_ip: String,
    #[serde(rename = "WANIPv6")]
    wan_ipv6: String,
    #[serde(rename = "ProductSN")]
    product_sn: String,
    #[serde(rename = "DevType")]
    dev_type: String,
    #[serde(rename = "SWVer")]
    sw_ver: String,
    #[serde(rename = "ProductCls")]
    product_cls: String,
}

/// Represents a port forwarding rule, including its properties and settings.
#[derive(Debug, Deserialize, Clone)]
pub struct PortForwardingRule {
    #[serde(rename = "protocol")]
    protocol: String,
    #[serde(rename = "inPort")]
    in_port: u16,
    #[serde(rename = "enable")]
    enable: u8,
    #[serde(rename = "desp")]
    description: String,
    #[serde(rename = "client")]
    client: String,
    #[serde(rename = "exPort")]
    ex_port: u16,
}

/// Represents the data returned by the router when retrieving a list of port forwarding rules.
#[derive(Debug, Deserialize)]
pub struct PortForwardingData {
    #[serde(rename = "mask")]
    mask: String,
    #[serde(rename = "lanIp")]
    lan_ip: String,
    #[serde(rename = "count")]
    count: u32,
    #[serde(flatten)]
    rules: HashMap<String, PortForwardingRule>,
}

#[derive(Debug, Deserialize)]
pub struct ActionResult {
    #[serde(rename = "retVal")]
    ret_val: i32,
}

/// Represents the different actions that can be performed on a port forwarding rule.
#[derive(Debug, Deserialize)]
pub enum PortForwardingAction {
    Add,
    Enable,
    Disable,
    Delete,
}

impl PortForwardingAction {
    fn as_str(&self) -> &str {
        match self {
            PortForwardingAction::Add => "add",
            PortForwardingAction::Enable => "enable",
            PortForwardingAction::Disable => "disable",
            PortForwardingAction::Delete => "del",
        }
    }
}

impl Tianyi {
    async fn rand_str() -> String {
        let mut rng = rand::thread_rng();
        rng.gen::<f64>().to_string()
    }

    /// Creates a new `Tianyi` instance with the provided `username` and `password`.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is a problem connecting to the router or logging in.
    async fn new(ip: &str, username: &str, password: &str) -> Result<Self> {
        let url = format!("http://{}", ip);
        let proxy = Proxy::http("http://127.0.0.1:8083")?;
        let client = Client::builder()
            .proxy(proxy)
            .cookie_store(true)
            .build()?;
        let login_payload = [("username", username), ("psd", password)];
        let response = client.post(&format!("{}/cgi-bin/luci", url))
            .form(&login_payload)
            .send()
            .await?;

        let token = match response.text().await {
            Ok(text) => {
                let re = regex::Regex::new(r"token: '([a-z0-9]{32})'").unwrap();
                re.captures(&text).context("Failed to parse token")?[1].to_string()
            }
            Err(err) => return Err(err.into()),
        };

        Ok(Tianyi { url, client, token })
    }

    /// Logs out from the router.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is a problem connecting to the router.
    pub async fn logout(&self) -> Result<()> {
        let payload = [("token", &self.token), ("_", &Self::rand_str().await)];

        let response = self.client.post(&format!("{}/cgi-bin/luci/admin/logout", self.url))
            .form(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to logout"))
        }
    }

    /// Retrieves gateway information from the router.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is a problem connecting to the router or parsing the response.
    pub async fn gwinfo(&self) -> Result<GatewayInfo> {
        let payload = [("get", "part"), ("_", &Self::rand_str().await)];

        let response = self.client.get(&format!("{}/cgi-bin/luci/admin/settings/gwinfo", self.url))
            .query(&payload)
            .send()
            .await?;

        let gw_info: GatewayInfo = response.json().await?;
        Ok(gw_info)
    }

    /// Retrieves a list of port forwarding rules from the router.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is a problem connecting to the router or parsing the response.
    pub async fn port_forwarding(&self) -> Result<PortForwardingData> {
        let payload = [("_", &Self::rand_str().await)];

        let response = self.client.get(&format!("{}/cgi-bin/luci/admin/settings/pmDisplay", self.url))
            .query(&payload)
            .send()
            .await?;

        let port_forwarding_data: PortForwardingData = response.json().await?;
        Ok(port_forwarding_data)
    }

    pub async fn get_port_forwarding_rules(&self) -> Result<Vec<PortForwardingRule>> {
        let port_forwarding_data = self.port_forwarding().await?;
        let rules = port_forwarding_data.rules.into_iter().map(|(_, rule)| rule).collect();
        Ok(rules)
    }

    /// Sets a port forwarding rule on the router.
    ///
    /// # Arguments
    ///
    /// * `action` - The desired action to perform on the rule (add, delete, enable, or disable).
    /// * `rule_desp` - The description of the rule to be modified.
    /// * `rule` - An optional `PortForwardingRule` containing the new rule settings.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is a problem connecting to the router or performing 
    pub async fn set_port_forwarding_rule(&self, action: PortForwardingAction, srvname: &str, rule: Option<&PortForwardingRule>) -> Result<ActionResult> {
        let rand_str = Self::rand_str().await;
        
        let mut payload = vec![
            ("srvname", srvname),
            ("token", &self.token),
            ("op", action.as_str()),
            ("_", &rand_str),
        ];

        let ex_port = rule.map_or("".to_owned(), |rule| rule.ex_port.to_string());
        let in_port = rule.map_or("".to_owned(), |rule| rule.in_port.to_string());

        if let Some(rule) = rule {
            payload.push(("client", &rule.client));
            payload.push(("protocol", &rule.protocol));
            payload.push(("exPort", &ex_port));
            payload.push(("inPort", &in_port));
        }

        let response = self.client.post(&format!("{}/cgi-bin/luci/admin/settings/pmSetSingle", self.url))
            .form(&payload)
            .send()
            .await?;

        let action_result: ActionResult = response.json().await?;
        Ok(action_result)
    }


    /// Updates a port forwarding rule with a new IP address.
    ///
    /// # Arguments
    ///
    /// * `old_ip` - The original IP address to be updated.
    /// * `new_ip` - The new IP address to replace the original IP address.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if there is a problem connecting to the router or updating the rule.
    pub async fn update_port_forwarding_rule(
        &self,
        old_ip: &str,
        new_ip: &str,
    ) -> Result<()> {
        let rules = self.get_port_forwarding_rules().await?;
    
        let mut updated_rules = Vec::new();
        for rule in rules {
            if rule.client == old_ip {
                let mut updated_rule = rule.clone();
                updated_rule.client = new_ip.to_string();
                updated_rules.push(updated_rule);
    
                self.set_port_forwarding_rule(PortForwardingAction::Delete, &rule.description, Some(&rule))
                    .await?;
            }
        }
    
        for updated_rule in updated_rules {
            self.set_port_forwarding_rule(PortForwardingAction::Add, &updated_rule.description, Some(&updated_rule))
                .await?;
        }
    
        Ok(())
    }

}