use std::error::Error;
use std::fmt;
use std::ops::{Index, IndexMut, /*RangeBounds*/};

use super::command;

#[derive(Debug)]
pub enum TapeError {
    Eof,
    OutOfBounds,
    InvalidArgument,
}

impl fmt::Display for TapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TapeError::*;
        write!(
            f,
            "{}",
            match self {
                Eof => "EOF",
                OutOfBounds => "Out of Bounds",
                InvalidArgument => "Invalid Argument",
            }
        )
    }
}

impl Error for TapeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Tape<T: Copy> {
    data: Vec<T>,
    cursor: usize,
}

/*
impl<I: RangeBounds<usize>, T: Copy> Index<I> for Tape<T> {
    type Output = [T];

    fn index(&self, bounds: I) -> &[T] {
        use std::ops::Bound::*;

        let min = match bounds.start_bound() {
            Included(i) => i,
            Excluded(i) => i + 1,
            Unbounded => 0,
        };

        let max = match bounds.end_bound() {
            Included(i) => i,
            Excluded(i) => i - 1,
            Unbounded => usize::MAX,
        };

        &self.data[min..=max]
    }
}
// */

//*
impl<T: Copy> Index<usize> for Tape<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}

impl<T: Copy> IndexMut<usize> for Tape<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}
// */

impl<T: Copy> Tape<T> {
    pub fn new(data: Vec<T>) -> Tape<T> {
        Tape { data, cursor: 0 }
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    pub fn jump_relative(&mut self, target: isize) {
        let c = self.get_cursor() as isize;
        self.jump((c + target) as usize);
    }

    pub fn jump(&mut self, target: usize) {
        if target >= self.data.len() {
            panic!("Tape pointer jump target {} is outside right bound", target)
        };
        self.cursor = target;
    }

    pub fn peek(&self) -> T {
        self[self.cursor]
    }

    pub fn peek_at(&self, index: usize) -> Result<T, TapeError> {
        if index <= self.len() {
            Ok(self[index])
        } else {
            Err(TapeError::OutOfBounds)
        }
    }

    pub fn read_inc(&mut self) -> (T, bool) {
        let n = self.peek();
        (n, self.inc_cursor())
    }

    /*
        pub fn inc_read(&mut self) -> Result<T, TapeError> {
            if self.inc_cursor() {
                Ok(self[self.cursor])
            } else {
                Err(TapeError::Eof)
            }
        }
    */

    pub fn inc_cursor(&mut self) -> bool {
        if self.cursor == self.len() {
            return false;
        }
        self.cursor += 1;
        true
    }

    pub fn peek_relative(&mut self, offset: isize) -> T {
        let target = self.get_cursor() as isize + offset;
        if target < 0 || (target as usize) >= self.data.len() {
            panic!("Tape peek relative target {} is outside bounds", target)
        };
        self[target as usize]
    }

    pub fn write_at(&mut self, index: usize, value: T) {
        self[index] = value;
    }

    pub fn write(&mut self, value: T) {
        let index = self.cursor;
        self[index] = value;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, n: T) {
        self.data.push(n);
    }

    pub fn pop(&mut self) -> (T, bool) {
        /*if self.len() == 0 or self.cursor < 0 {
            // what to do if tape is empty
        } else*/
        if self.cursor == self.len() - 1 {
            (self.peek(), true)
        } else {
            // note that this will not panic unless cursor points to outside
            // the tape, which should not normally happen, unless len == 0
            (self.data.pop().unwrap(), false)
        }
    }

    pub fn _raw_pop(&mut self) -> T {
        self.data.pop().unwrap()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

// TODO: change this to not require Clone when Vec.resize_with() is out of nightly
impl<T: Default + Copy> Tape<T> {
    pub fn move_cursor(&mut self, change: isize) -> bool {
        let m = self.cursor as isize + change;
        if m < 0 {
            panic!("Tape pointer outside left bound")
        };
        let outside_right_bound = m >= self.data.len() as isize;
        if outside_right_bound {
            self.grow((m + 1) as usize);
        };
        self.cursor = m as usize;
        outside_right_bound
    }

    pub fn grow(&mut self, new_size: usize) -> () {
        if new_size > self.data.len() {
            self.data.resize(new_size, T::default())
        }
    }
}

impl Tape<u8> {
    pub fn _peek_u32(&self, index: usize) -> Result<u32, TapeError> {
        use std::convert::TryInto;

        if index >= self.data.len() {
            //~ self.grow(index + 1);
            return Err(TapeError::OutOfBounds);
        }

        let bytes = &self.data[index..(index+4)];
        Ok(u32::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn i8_subtract(&mut self, n: i8) -> bool {
        let m = self.data[self.cursor] as i8;
        let (v, overflow) = m.overflowing_add(-n);
        self.data[self.cursor] = v as u8;
        overflow
    }

    pub fn read_int(&mut self, bytes: usize) -> Result<u32, TapeError> {
        if bytes == 0 {
            Err(TapeError::InvalidArgument)
        } else if bytes > 4 {
            Err(TapeError::InvalidArgument)
        } else {
            let mut n: u32 = 0;
            for _i in 0..bytes {
                let (byte, success) = self.read_inc();
                let byte = byte as u8;
                n = (n << 8) | (byte as u32);
                if !success {
                    return Err(TapeError::OutOfBounds);
                }
            }
            Ok(n)
        }
    }

    pub fn peek_int(&self, mut index: usize, bytes: usize) -> Result<u32, TapeError> {
        if bytes == 0 {
            Err(TapeError::InvalidArgument)
        } else if bytes > 4 {
            Err(TapeError::InvalidArgument)
        } else {
            let mut n: u32 = 0;
            for _i in 0..bytes {
                let byte = self.peek_at(index)? as u8;
                index += 1;
                n = (n << 8) | (byte as u32);
            }
            Ok(n)
        }
    }

    pub fn push_int(&mut self, bytes: usize, n: u32) {
        if bytes == 0 {
            return;
        } else if bytes > 4 {
            return;
        } else {
            for i in 0..bytes {
                let shift = 8 * (bytes - i - 1);
                self.push(((n & (0xff << shift)) >> shift) as u8);
            }
        }
    }

    pub fn write_int_at(&mut self, index: usize, bytes: usize, n: u32) {
        if bytes == 0 {
            panic!();
        } else if bytes > 4 {
            panic!();
        } else {
            for i in 0..bytes {
                let shift = 8 * (bytes - i - 1);
                self.write_at(index + i, ((n & (0xff << shift)) >> shift) as u8);
            }
        }
    }
}

impl fmt::Display for Tape<u8> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn format_command(bytecode: &Tape<u8>, index: usize) -> Option<(String, usize)> {
            //use command::Opcode::*;

            let com =
                command::Opcode::from(bytecode.peek_at(index).expect("Unexpected end of tape"));

            let length = com.len();

            if index + length - 1 >= bytecode.len() {
                return None;
            }

            let mut s = format!("{:08x}: ", index);
            for i in 0..length {
                s = format!(
                    "{} {:02x}",
                    s,
                    bytecode.peek_at(index + i).expect("Unexpected end of tape") as u8
                );
            }

            s = format!("{:26}", s);

            if length == 1 {
                s = format!("{} {:?}\n", s, com);
            } else {
                s = format!(
                    "{} {:?} {}\n",
                    s,
                    com,
                    bytecode
                        .peek_int(index + 1, length - 1)
                        .expect("Unexpected end of tape") as i32
                );
            }

            //s = format!("{}\n", s);

            Some((s, length))
        }

        let mut full_output = String::new();

        let mut index = 0;
        while index < self.len() {
            if let Some((s, n)) = format_command(&self, index) {
                full_output += &s;
                index += n;
            } else {
                full_output += "((Error))\n";
                break;
            }
        }

        // Write into the supplied output stream: `f`.
        // Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!`
        // uses syntax which is very similar to `println!`.
        write!(f, "{}", full_output)
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn u8_test() {
    }
}

