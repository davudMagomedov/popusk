use crate::error_ext::ComResult;
use crate::id::ID;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use memmap2::MmapMut;

const BLOCK_SIZE: usize = 512; // bytes
const AIL_FILE: &str = "ail";

fn find_zero_in_byte(byte: u8) -> usize {
    if byte & 0b00000001 == 0 {
        return 0;
    } else if byte & 0b00000010 == 0 {
        return 1;
    } else if byte & 0b00000100 == 0 {
        return 2;
    } else if byte & 0b00001000 == 0 {
        return 3;
    } else if byte & 0b00010000 == 0 {
        return 4;
    } else if byte & 0b00100000 == 0 {
        return 5;
    } else if byte & 0b01000000 == 0 {
        return 6;
    } else if byte & 0b10000000 == 0 {
        return 7;
    } else {
        unreachable!();
    }
}

fn set_one_in_byte(byte: u8, index: usize) -> u8 {
    byte | (1 << index)
}

fn set_zero_in_byte(byte: u8, index: usize) -> u8 {
    byte & !(1 << index)
}

pub struct AvailableIDList {
    path: PathBuf,
    // Markup:
    // [blocks]
    mapped: MmapMut,
}

impl AvailableIDList {
    pub fn open(working_dir: &Path) -> ComResult<Self> {
        let path = working_dir.join(AIL_FILE);
        let file = OpenOptions::new().read(true).write(true).open(&path)?;

        Ok(AvailableIDList {
            path,
            mapped: unsafe { MmapMut::map_mut(&file)? },
        })
    }

    pub fn create(working_dir: &Path) -> ComResult<Self> {
        let path = working_dir.join(AIL_FILE);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)?;

        file.write_all(&[0; BLOCK_SIZE])?;

        Ok(AvailableIDList {
            path,
            mapped: unsafe { MmapMut::map_mut(&file)? },
        })
    }

    pub fn grab_id(&mut self) -> ComResult<ID> {
        let (byte_index, byte) = match self
            .byte_section()
            .into_iter()
            .enumerate()
            .find(|(_, &byte)| byte != u8::MAX)
        {
            None => {
                let index = self.byte_section().len();
                self.grow()?;

                (index, self.mapped[index])
            }
            Some((index, &byte)) => (index, byte),
        };

        let index_in_byte = find_zero_in_byte(byte);
        self.byte_section_mut()[byte_index] = set_one_in_byte(byte, index_in_byte);

        return Ok(ID::new((8 * byte_index + index_in_byte) as u64));
    }

    pub fn release_id(&mut self, id: ID) -> ComResult<()> {
        // 8 - count of bits in a byte.
        if id.value() < (self.byte_count() * 8) as u64 {
            let byte_index = id.value() / 8;
            let index_in_byte = id.value() % 8;

            self.byte_section_mut()[byte_index as usize] = set_zero_in_byte(
                self.byte_section()[byte_index as usize],
                index_in_byte as usize,
            );
        }

        Ok(())
    }

    fn grow(&mut self) -> ComResult<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&self.path)?;

        file.write_all(&[0; BLOCK_SIZE])?;
        file.flush()?;

        self.mapped = unsafe { MmapMut::map_mut(&file)? };

        Ok(())
    }

    fn byte_section(&self) -> &[u8] {
        &self.mapped[..]
    }

    fn byte_section_mut(&mut self) -> &mut [u8] {
        &mut self.mapped[..]
    }

    fn byte_count(&self) -> usize {
        self.mapped.len()
    }
}
