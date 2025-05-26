pub mod documents;
pub mod api;

pub fn is_expected_media_type(media_type: &str) -> bool {
    match media_type {
        "TV" | "Movie" | "OVA" | "ONA" => true,
        _ => false
    }
}
