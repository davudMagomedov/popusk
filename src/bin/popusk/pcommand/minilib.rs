use super::{PEResult, PExecError};

use popusk::comps_appearance::{entitytype_to_string, ETYPE_REGULAR_OR_DOCUMENT};
use popusk::types::{verify_tags as verify_tags_popusk, EntityType, LibEntityMut, Tag};

use itertools::Itertools;

pub(super) fn verify_tags(tags: &[Tag]) -> PEResult<()> {
    verify_tags_popusk(tags).map_err(|(invalid_tags, regex)| PExecError::InvalidTags {
        sepd_tags: invalid_tags
            .into_iter()
            .map(|s| format!("'{s}'"))
            .join(", "),
        regex,
    })
}

pub(super) fn verify_etype(etype: EntityType, libentity: &LibEntityMut) -> PEResult<()> {
    use EntityType::*;
    match (libentity.etype(), etype) {
        (Section, Regular | Document) => Err(PExecError::InvalidEtype {
            expected: entitytype_to_string(Section),
            actually: entitytype_to_string(etype),
        }),
        (Regular | Document, Section) => Err(PExecError::InvalidEtype {
            expected: ETYPE_REGULAR_OR_DOCUMENT,
            actually: entitytype_to_string(Section),
        }),
        _ => Ok(()),
    }
}
