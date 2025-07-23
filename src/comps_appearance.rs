use crate::error_ext::{ComError, ComResult};
use crate::types::{EntityBase, EntityType, Progress, ProgressUpdate, Tag};

pub const ETYPE_SECTION: &str = "section";
pub const ETYPE_REGULAR: &str = "regular";
pub const ETYPE_DOCUMENT: &str = "document";
pub const ETYPE_REGULAR_OR_DOCUMENT: &str = "regular or document";

/// Returns string-equivalet for `EntityType` *in lower case*.
#[inline]
pub const fn entitytype_to_string(etype: EntityType) -> &'static str {
    match etype {
        EntityType::Section => ETYPE_SECTION,
        EntityType::Regular => ETYPE_REGULAR,
        EntityType::Document => ETYPE_DOCUMENT,
    }
}

pub fn entitytype_from_string(string: &str) -> ComResult<EntityType> {
    match string {
        ETYPE_DOCUMENT => Ok(EntityType::Document),
        ETYPE_REGULAR => Ok(EntityType::Regular),
        ETYPE_SECTION => Ok(EntityType::Section),
        _ => Err(ComError::from(format!(
            "couldn't recognize '{}' as entity type",
            string
        ))),
    }
}

pub fn parse_string_to_tags(stringifed_tags: &str) -> ComResult<Vec<Tag>> {
    Ok(stringifed_tags
        .split_whitespace()
        .map(|tag| tag.trim().to_string())
        .collect())
}

pub fn entitybase_to_oneline_string(entitybase: &EntityBase) -> String {
    format!("{{ name: '{}', ... }}", entitybase.name)
}

pub fn tags_to_string(tags: &[Tag]) -> String {
    if tags.len() == 0 {
        return "<no tags>".to_string();
    }

    let mut tag_iter = tags.iter();
    let tag_iter_first = tag_iter.next().unwrap().clone();

    tag_iter.fold(tag_iter_first, |l, r| format!("{} {}", l, r))
}

pub fn entitybase_to_fullinfo_string(entitybase: &EntityBase) -> String {
    format!(
        "Name: '{}'\nType: {}\nTags: {}",
        entitybase.name,
        entitytype_to_string(entitybase.etype),
        tags_to_string(&entitybase.tags),
    )
}

pub fn progress_to_string(progress: &Progress) -> String {
    format!("{}/{}", progress.passed(), progress.ceiling())
}

pub fn progress_from_string(s: &str) -> ComResult<Progress> {
    const PASSED_CEILING_SEP: char = '/';

    let separator_posisiton = s
        .find(PASSED_CEILING_SEP)
        .ok_or_else(|| format!("use syntax 'passed{}ceiling'", PASSED_CEILING_SEP))?;

    let passed = s[..separator_posisiton].parse::<usize>()?;
    let ceiling = s[separator_posisiton + 1..].parse::<usize>()?;

    if passed > ceiling {
        return Err("<passed> must be less than <ceiling>".into());
    }

    Ok(Progress::with_passed(passed, ceiling))
}

fn parse_string_to_integer(string: &str) -> ComResult<usize> {
    match string.parse::<usize>() {
        Ok(int) => Ok(int),
        Err(_) => Err(format!("couldn't parse string to integer: {}", string).into()),
    }
}

pub fn progress_update_from_string(s: &str) -> ComResult<ProgressUpdate> {
    match s.get(0..1) {
        Some("+") => match s.get(1..) {
            Some(stried_number) => Ok(ProgressUpdate::increase(parse_string_to_integer(
                stried_number,
            )?)),
            None => unreachable!(), // Cause we've succesfully got `s.get(0..1)` above.
        },
        Some("-") => match s.get(1..) {
            Some(stried_number) => Ok(ProgressUpdate::decrease(parse_string_to_integer(
                stried_number,
            )?)),
            None => unreachable!(), // Cause we've succesfully got `s.get(0..1)` above.
        },
        Some(_) => Ok(ProgressUpdate::set(parse_string_to_integer(s)?)),
        None => Err(format!("couldn't recognize string '{}' as valid", s).into()),
    }
}
