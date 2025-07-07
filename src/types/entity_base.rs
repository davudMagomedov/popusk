use once_cell::sync::OnceCell;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

pub type Tag = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "regular")]
    Regular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityBase {
    pub name: String,
    pub etype: EntityType,
    pub tags: Vec<Tag>,
}

const TAG_REGEX_STRING: &str = r"^[\w-]+$";
static TAG_REGEX: OnceCell<Regex> = OnceCell::new();

/// Returns `true` if given string is valid tag.
pub fn verify_tag(tag: &str) -> bool {
    TAG_REGEX
        .get_or_init(|| unsafe { Regex::new(TAG_REGEX_STRING).unwrap_unchecked() })
        .is_match(tag)
}

/// Returns Ok(()) if all tags are good, otherwise returns the bad ones and regular expression they
/// doesn't match.
pub fn verify_tags(tags: &[Tag]) -> Result<(), (Vec<&Tag>, &'static str)> {
    let bad_tags: Vec<_> = tags.into_iter().filter(|&tag| !verify_tag(tag)).collect();
    match bad_tags.is_empty() {
        true => Ok(()),
        false => Err((bad_tags, r"[\w-]+")),
    }
}
