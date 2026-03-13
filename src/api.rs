use anyhow::{bail, Context, Result};
use reqwest::{Client, Method};
use serde_json::Value;
use std::time::Duration;

pub struct ProxmoxClient {
    client: Client,
    base_url: String,
    node: String,
    ticket: String,
    csrf_token: String,
}

impl ProxmoxClient {
    pub async fn connect(config: &crate::config::ProxmoxConfig) -> Result<Self> {
        let client = Client::builder()
            .danger_accept_invalid_certs(!config.verify_ssl)
            .timeout(Duration::from_secs(60))
            .build()?;

        let base_url = format!("https://{}:{}/api2/json", config.host, config.port);

        let resp: Value = client
            .post(format!("{}/access/ticket", base_url))
            .form(&[("username", &config.user), ("password", &config.password)])
            .send()
            .await
            .context("Failed to connect to Proxmox API")?
            .json()
            .await?;

        let data = resp.get("data").context("No data in auth response")?;
        let ticket = data["ticket"]
            .as_str()
            .context("No ticket in auth response")?
            .to_string();
        let csrf_token = data["CSRFPreventionToken"]
            .as_str()
            .context("No CSRF token in auth response")?
            .to_string();

        Ok(Self {
            client,
            base_url,
            node: config.node.clone(),
            ticket,
            csrf_token,
        })
    }

    pub fn node(&self) -> &str {
        &self.node
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Value> {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self
            .client
            .request(method.clone(), &url)
            .header("Cookie", format!("PVEAuthCookie={}", self.ticket));

        if method != Method::GET {
            req = req.header("CSRFPreventionToken", &self.csrf_token);
        }

        if let Some(params) = params {
            req = req.form(params);
        }

        let resp = req.send().await.context("API request failed")?;
        let status = resp.status();
        let body: Value = resp.json().await.context("Failed to parse API response")?;

        if !status.is_success() {
            let message = body["message"].as_str().unwrap_or("Unknown error");
            let errors = body.get("errors").map(|e| e.to_string()).unwrap_or_default();
            if errors.is_empty() {
                bail!("API error {}: {}", status.as_u16(), message);
            } else {
                bail!("API error {}: {} — {}", status.as_u16(), message, errors);
            }
        }

        Ok(body.get("data").cloned().unwrap_or(Value::Null))
    }

    pub async fn get(&self, path: &str) -> Result<Value> {
        self.request(Method::GET, path, None).await
    }

    pub async fn post(&self, path: &str, params: &[(&str, &str)]) -> Result<Value> {
        self.request(Method::POST, path, Some(params)).await
    }

    pub async fn put(&self, path: &str, params: &[(&str, &str)]) -> Result<Value> {
        self.request(Method::PUT, path, Some(params)).await
    }

    pub async fn delete(&self, path: &str) -> Result<Value> {
        self.request(Method::DELETE, path, None).await
    }

    pub async fn wait_task(&self, upid: &str) -> Result<()> {
        loop {
            let status = self
                .get(&format!("/nodes/{}/tasks/{}/status", self.node, upid))
                .await?;
            match status["status"].as_str() {
                Some("stopped") => {
                    let exit = status["exitstatus"].as_str().unwrap_or("");
                    if exit == "OK" {
                        return Ok(());
                    }
                    bail!("Task failed: {}", exit);
                }
                _ => tokio::time::sleep(Duration::from_secs(1)).await,
            }
        }
    }
}
