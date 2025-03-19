pub(crate) mod anime;
pub(crate) mod anime_search;

pub(crate) fn is_expected_media_type(media_type: &str) -> bool {
    match media_type {
        "TV" | "Movie" | "OVA" | "ONA" => true,
        _ => false
    }
}
