#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Character {
    pub(crate) mal_id: i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CharacterCast {
    pub(crate) character: Character
    // voice_actors etc...
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AniCharaBridge {
    pub(crate) mal_id: i64,
    pub(crate) characters: Vec<CharacterCast>
}
