use std::{path::PathBuf, str::FromStr, time::Duration};
use futures::TryStreamExt;
use tokio::{fs, io::{AsyncWriteExt, BufWriter}, time};
use serde_json::Value;
use reqwest::{Client as HttpClient, StatusCode};
use tracing::info;
use anyhow::bail;

pub async fn request(
    http_client: &HttpClient, 
    timeout: Duration,
    url: &str
) 
-> anyhow::Result<(Vec<Value>, Value)> {
    info!("requesting {url}");
    let res = http_client.get(url).timeout(timeout).send().await?;
    let status = res.status();
    if status != StatusCode::OK {
        bail!("{url} responsed {status}");
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

pub fn paged_url(url: &str, page: u32) -> String {
    match page {
        0..=1 => url.to_string(),
        _ => format!("{url}?page={page}")
    }
}

pub async fn request_pages(
    http_client: &HttpClient,
    interval: Duration,
    timeout: Duration,
    url: &str
) -> anyhow::Result<Vec<Value>> {
    let mut list = vec![];
    let mut page = 0;

    loop {
        page += 1;
        
        let url = paged_url(url, page);
        let (mut data, pagination) = request(http_client, timeout, &url).await?;
        list.append(&mut data);

        if !matches!(pagination["has_next_page"], Value::Bool(true)) {
            break;
        }

        time::sleep(interval).await;
    }

    Ok(list)
}

pub async fn request_img(
    http_client: &HttpClient,
    timeout: Duration,
    url: &str,
    file_name: &str,
) -> anyhow::Result<()> {
    info!("requesting {url}");

    let res = http_client.get(url).timeout(timeout).send().await?;
    if res.status() != StatusCode::OK {
        bail!("failed to request img");
    }

    let file_name = PathBuf::from_str(file_name)?;
    if let Some(dir) = file_name.parent() {
        fs::create_dir_all(dir).await?;
    }

    let file = fs::File::create(&file_name).await?;
    let mut write_stream = BufWriter::new(file);
    let mut read_stream = res.bytes_stream();
    
    while let Some(b) = read_stream.try_next().await? {
        write_stream.write(&b).await?;
    }
    write_stream.flush().await?;

    info!("created {file_name:?}");
    Ok(())
}
