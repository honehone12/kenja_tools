
use std::time::Duration;
use tokio::time;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use reqwest::{Client as HttpClient, StatusCode};
use tracing::info;
use anyhow::bail;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AnimeText {
    pub anime: AnimeSimple,
    pub characters: Vec<CharacterSimple>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CharacterSimple {
    pub mal_id: i64,
    pub name: Option<String>,
    pub name_kanji: Option<String>,
    pub about: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AnimeSimple {
    pub mal_id: i64,
    pub title: Option<String>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub synopsis: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AnimeCharacters {
    pub mal_id: i64,
    pub characters: Vec<Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Season {
    pub year: u32,
    pub seasons: Vec<String>
}

pub(crate) fn paged_url(path: &str, page: u32) -> String {
    match page {
        0..=1 => path.to_string(),
        _ => format!("{path}?page={page}")
    }
}

pub(crate) async fn request(http_client: &HttpClient, url: &str) 
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
