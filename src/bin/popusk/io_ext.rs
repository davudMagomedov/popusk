use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult};

const STDIN_NONUTF8_ERROR_MSG: &str = "shitty utf8 sequence from stdin";

pub trait IoExt {
    fn read_to_end_string(&mut self) -> IoResult<String>;
}

impl<T: Read> IoExt for T {
    fn read_to_end_string(&mut self) -> IoResult<String> {
        let mut vector = Vec::new();
        self.read_to_end(&mut vector)?;

        String::from_utf8(vector)
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, STDIN_NONUTF8_ERROR_MSG))
    }
}
