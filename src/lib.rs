use std::time::Duration;
use tokio::time;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use reqwest::{Client as HttpClient, StatusCode};
use tracing::info;
use anyhow::bail;

#[derive(Serialize, Deserialize)]
pub struct Season {
    pub year: u32,
    pub seasons: Vec<String>
}

pub fn paged_url(path: &str, page: u32) -> String {
    match page {
        0..=1 => path.to_string(),
        _ => format!("{path}?page={page}")
    }
}

pub async fn request(http_client: &HttpClient, url: &str) 
-> anyhow::Result<(Vec<Value>, Value)> {
    info!("requesting {url}");
    let res = http_client.get(url).send().await?;
    let status = res.status();
    if status != StatusCode::OK {
        bail!("{url} respnsed {status}");
    }

    let text = res.text().await?;
    let mut json = serde_json::from_str::<Value>(&text)?;

    let mut data = vec![];
    match json["data"] {
        Value::Null => bail!("property 'data' not found"),
        ref mut v => {
            if let Some(a) = v.as_array_mut() {
                data.append(a);
            } else if !v.is_null() {
                data.push(v.take());
            }
        }
    };
    let pagination = json["pagination"].take();

    Ok((data, pagination))
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
        
        let url = paged_url(path, page);
        let (mut data, pagination) = request(http_client, &url).await?;
        list.append(&mut data);

        if !matches!(pagination["has_next_page"], Value::Bool(true)) {
            break;
        }

        time::sleep(interval).await;
    }

    Ok(list)
}