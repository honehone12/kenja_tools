use std::{path::PathBuf, str::FromStr};
use url::Url;
use tokio::fs;
use tracing::warn;

use crate::documents::anime_search::ItemType;

const RX: &'static str = "Rx";
const HENTAI: &'static str = "Hentai";

#[inline]
pub fn is_hentai_str(rating_str: &str) -> bool {
    return rating_str.contains(RX) || rating_str.contains(HENTAI)
}

#[inline]
pub fn is_expected_media_type(media_type: &str) -> bool {
    match media_type {
        "TV" | "Movie" | "OVA" | "ONA" => true,
        _ => false
    }
}

pub async fn create_new_img(
    raw_img_root: &str,
    exist_img_root: &str, 
    new_img_root: &str,
    img_url: &str,
    item_type: ItemType
) -> anyhow::Result<Option<String>> {
    
    let u = Url::parse(img_url)?;
    let mut p = u.path().to_string();
    p.remove(0);
    let path = PathBuf::from_str(raw_img_root)?.join(p);

    if !fs::try_exists(&path).await? {
        warn!("file {path:?} does not exits");
        return Ok(None);
    }
    
    let hash = blake3::hash(img_url.as_bytes());
    let hash = hash.as_bytes();
    let hex = hex::encode(&hash[..16]);

    let new_file = format!("preview/{item_type}/{hex}.jpg");
    let new_path = PathBuf::from_str(exist_img_root)?.join(&new_file);
    if fs::try_exists(new_path).await? {
        return Ok(Some(new_file))
    }
    
    let new_path = PathBuf::from_str(new_img_root)?.join(&new_file);
    fs::copy(path, new_path).await?;

    Ok(Some(new_file))
}
