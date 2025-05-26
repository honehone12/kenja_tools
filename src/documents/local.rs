use serde::{Deserialize, Serialize};
use super::id::ItemId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Img {
    pub item_id: ItemId,
    pub img: String,
    pub path: Option<String>
}
