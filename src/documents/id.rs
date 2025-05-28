use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(i32)]
pub enum ItemType {
    Anime = 1,
    Character = 2
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct ItemId {
    pub id: i64,
    pub item_type: ItemType
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ids {
    pub _id: ObjectId,
    pub item_id: ItemId
}
