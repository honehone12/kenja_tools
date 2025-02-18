use std::time::Duration;
use tokio::time;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use reqwest::{Client as HttpClient, StatusCode};
use tracing::{info, warn};
use anyhow::bail;

#[derive(Serialize, Deserialize)]
pub struct Season {
    pub year: u32,
    pub seasons: Vec<String>
}

pub async fn request_pages(
    http_client: &HttpClient,
    path: &str,
    interval_mil: u64
) -> anyhow::Result<Vec<Value>> {
    let mut list = vec![];
    let mut page = 0;
    let interval = Duration::from_millis(interval_mil);

    loop {
        page += 1;
        
        let url = match page {
            0..=1 => path.to_string(),
            _ => format!("{path}?page={page}")
        };

        info!("requesting {url}");
        let res = http_client.get(&url).send().await?;
        let status = res.status();
        if status != StatusCode::OK {
            bail!("{url} respnsed {status}");
        }

        let text = res.text().await?;
        let mut json = serde_json::from_str::<Value>(&text)?;

        match json["data"] {
            Value::Null => bail!("property 'data' not found"),
            ref mut v => {
                if let Some(l) = v.as_array_mut() {
                    list.append(l);
                } else {
                    list.push(v.take());
                }
            }
        };
        match json["pagination"] {
            Value::Null => {
                warn!("property 'pagination' not found");
                break;
            }
            ref p => {
                time::sleep(interval).await;
                
                if !matches!(p["has_next_page"], Value::Bool(true)) {
                    break;
                }
            }
        };
    }

    Ok(list)
}