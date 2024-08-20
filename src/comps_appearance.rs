use crate::entity_base::{EntityBase, EntityType, Tag};
use crate::error_ext::ComResult;
use crate::progress::Progress;
use crate::progress_update::ProgressUpdate;

const STRINGIFIED_ETYPE_SECTION: &str = "section";
const STRINGIFIED_ETYPE_REGULAR: &str = "regular";
const STRINGIFIED_ETYPE_DOCUMENT: &str = "document";

/// Returns string-equivalet for `EntityType` *in lower case*.
#[inline]
pub fn entitytype_to_string(etype: EntityType) -> String {
    match etype {
        EntityType::Section => STRINGIFIED_ETYPE_SECTION.to_string(),
        EntityType::Regular => STRINGIFIED_ETYPE_REGULAR.to_string(),
        EntityType::Document => STRINGIFIED_ETYPE_DOCUMENT.to_string(),
    }
}

pub fn parse_string_to_tags(stringifed_tags: &str) -> ComResult<Vec<Tag>> {
    Ok(stringifed_tags
        .split(',')
        .map(|tag| tag.trim().to_string())
        .collect())
}

pub fn entitybase_to_oneline_string(entitybase: &EntityBase) -> String {
    format!(
        "{{ id: {}, name: '{}' }}",
        entitybase.id(),
        entitybase.name()
    )
}

fn tags_to_string(tags: &[Tag]) -> String {
    if tags.len() == 0 {
        return "<no tags>".to_string();
    }

    let mut tag_iter = tags.iter();
    let tag_iter_first = tag_iter.next().unwrap().clone();

    tag_iter.fold(tag_iter_first, |l, r| format!("{}, {}", l, r))
}

pub fn entitybase_to_fullinfo_string(entitybase: &EntityBase) -> String {
    format!(
        "Name: '{}'\nID: {}\nType: {}\nTags: {}",
        entitybase.name(),
        entitybase.id(),
        entitytype_to_string(entitybase.etype()),
        tags_to_string(entitybase.tags()),
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
        Some(plus) if plus == "+" => match s.get(1..) {
            Some(stried_number) => Ok(ProgressUpdate::increase(parse_string_to_integer(
                stried_number,
            )?)),
            None => unreachable!(), // Cause we've succesfully got `s.get(0..1)` above.
        },
        Some(minus) if minus == "-" => match s.get(1..) {
            Some(stried_number) => Ok(ProgressUpdate::decrease(parse_string_to_integer(
                stried_number,
            )?)),
            None => unreachable!(), // Cause we've succesfully got `s.get(0..1)` above.
        },
        Some(_) => Ok(ProgressUpdate::set(parse_string_to_integer(s)?)),
        None => Err(format!("couldn't recognize string '{}' as valid", s).into()),
    }
}
