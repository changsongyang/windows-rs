use super::*;

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

// A coded index (see codes.rs) is a table index that may refer to different tables. The size of the column in memory
// must therefore be large enough to hold an index for a row in the largest possible table. This function determines
// this size for the given winmd file.
pub fn coded_index_size(tables: &[usize]) -> usize {
    fn small(row_count: usize, bits: u8) -> bool {
        (row_count as u64) < (1u64 << (16 - bits))
    }

    fn bits_needed(value: usize) -> u8 {
        let mut value = value - 1;
        let mut bits: u8 = 1;
        while {
            value >>= 1;
            value != 0
        } {
            bits += 1;
        }
        bits
    }

    let bits_needed = bits_needed(tables.len());

    if tables.iter().all(|table| small(*table, bits_needed)) {
        2
    } else {
        4
    }
}

pub trait IntoStream {
    fn into_stream(self) -> Vec<u8>;
}

impl IntoStream for Vec<u8> {
    fn into_stream(mut self) -> Self {
        self.resize(round(self.len(), 4), 0);
        self
    }
}

pub trait Write {
    fn write_header<T: Sized>(&mut self, value: &T);
    fn write_u8(&mut self, value: u8);
    fn write_u16(&mut self, value: u16);
    fn write_u32(&mut self, value: u32);
    fn write_u64(&mut self, value: u64);
    fn write_code(&mut self, value: u32, size: usize);
    fn write_index(&mut self, index: u32, len: usize);
}

impl Write for Vec<u8> {
    fn write_header<T: Sized>(&mut self, value: &T) {
        unsafe {
            self.extend_from_slice(std::slice::from_raw_parts(
                value as *const _ as _,
                core::mem::size_of::<T>(),
            ));
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u16(&mut self, value: u16) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u32(&mut self, value: u32) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u64(&mut self, value: u64) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_code(&mut self, value: u32, size: usize) {
        if size == 2 {
            self.write_u16(value as u16);
        } else {
            self.write_u32(value);
        }
    }

    fn write_index(&mut self, index: u32, len: usize) {
        if len < (1 << 16) {
            self.write_u16(index as u16 + 1);
        } else {
            self.write_u32(index + 1);
        }
    }
}
