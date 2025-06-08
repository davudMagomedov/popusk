use std::ffi::{OsStr, OsString};
use std::fmt::Write;
use std::path::{Component, Path, PathBuf};

#[cfg(unix)]
const NAME_SEPARATOR: u8 = 0xA;

#[cfg(unix)]
/// Assumes that given path contains only names (not absolute path, no current/previous dir).
pub fn encode_path(path: &Path) -> OsString {
    let mut encoded_path = OsString::new();
    let mut components = path.components();
    encoded_path.push(components.next().unwrap());
    for component in components {
        let Component::Normal(name) = component else {
            unreachable!();
        };
        encoded_path.write_char(NAME_SEPARATOR as char).unwrap(); // no error can occur
        encoded_path.push(name);
    }
    encoded_path
}

#[cfg(unix)]
pub fn decode_path(st: &OsStr) -> PathBuf {
    use std::os::unix::ffi::OsStrExt;

    let st_bytes = st.as_bytes();
    let mut path = PathBuf::new();
    let mut start_index: usize = 0;
    let mut i: usize = 0;
    while i < st.len() {
        if st_bytes[i] == NAME_SEPARATOR {
            path = path.join(OsStr::from_bytes(&st_bytes[start_index..i]));
            start_index = i + 1;
        }

        i += 1;
    }

    path
}

#[cfg(windows)]
const THEMOSTRARE_SENTENCE: &[u8] = &['T' as u8, 'l' as u8, 'P' as u8];

#[cfg(windows)]
pub fn encode_path(path: &Path) -> OsString {
    let mut encoded_path = OsString::new();
    let mut components = path.components();
    encoded_path.push(components.next().unwrap());
    for component in components {
        let Component::Normal(name) = component else {
            unreachable!();
        };
        encoded_path
            .write_str(unsafe { std::str::from_utf8_unchecked(THEMOSTRARE_SENTENCE) })
            .unwrap(); // no error can occur
        encoded_path.push(name);
    }
    encoded_path
}

#[cfg(windows)]
pub fn decode_path(st: &OsStr) -> PathBuf {
    let st_bytes = st.as_encoded_bytes();
    let mut path = PathBuf::new();
    let mut start_index: usize = 0;
    let mut i: usize = 0;
    while i < st_bytes.len() {
        if &st_bytes[i..st_bytes.len().min(i + THEMOSTRARE_SENTENCE.len())] == THEMOSTRARE_SENTENCE
        {
            path = path
                .join(unsafe { OsStr::from_encoded_bytes_unchecked(&st_bytes[start_index..i]) });
            i += 3;
            start_index = i;
        } else {
            i += 1;
        }
    }

    path
}
