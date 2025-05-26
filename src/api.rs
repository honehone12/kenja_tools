use std::time::Duration;
use tokio::time;
use serde_json::Value;
use reqwest::{Client as HttpClient, StatusCode};
use tracing::info;
use anyhow::bail;

pub fn paged_url(url: &str, page: u32) -> String {
    match page {
        0..=1 => url.to_string(),
        _ => format!("{url}?page={page}")
    }
}

pub async fn request(http_client: HttpClient, url: &str) 
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
            } else {
                data.push(v.take());
            }
        }
    };
    let pagination = json["pagination"].take();

    Ok((data, pagination))
}

pub async fn request_pages(
    http_client: HttpClient,
    url: &str,
    interval: Duration
) -> anyhow::Result<Vec<Value>> {
    let mut list = vec![];
    let mut page = 0;

    loop {
        page += 1;
        
        let url = paged_url(url, page);
        let client = http_client.clone();
        let (mut data, pagination) = request(client, &url).await?;
        list.append(&mut data);

        if !matches!(pagination["has_next_page"], Value::Bool(true)) {
            break;
        }

        time::sleep(interval).await;
    }

    Ok(list)
}
