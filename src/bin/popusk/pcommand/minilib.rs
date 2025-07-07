use super::{PEResult, PExecError};

use popusk::{verify_tags as verify_tags_popusk, Tag};

use itertools::Itertools;

pub(super) fn verify_tags(tags: &[Tag]) -> PEResult<()> {
    verify_tags_popusk(tags).map_err(|(invalid_tags, regex)| PExecError::InvalidTags {
        sepd_tags: invalid_tags.into_iter().map(|s| format!("'{s}'")).join(", "),
        regex,
    })
}
