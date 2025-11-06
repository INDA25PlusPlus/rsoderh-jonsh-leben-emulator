use crate::instruction::{Register, RegisterPair};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ReadError<'c> {
    EndOfBuffer,
    UnexpectedChar(u8),
    UnexpectedSlice(&'c [u8]),
}

type Label<'a> = &'a [u8];

pub type ReadResult<'c, T> = Result<T, ReadError<'c>>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Reader<'a> {
    original: &'a [u8],
    buffer: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(slice: &[u8]) -> Reader {
        Reader {
            original: slice,
            buffer: slice,
        }
    }

    pub fn read(&mut self) -> Option<u8> {
        self.read_n(1).map(|slice| slice[0])
    }

    pub fn read_n<'b>(&'b mut self, n: usize) -> Option<&'a [u8]> {
        if self.buffer.len() < n {
            return None;
        }
        let (ret, rest) = self.buffer.split_at(n);
        self.buffer = rest;
        Some(ret)
    }

    pub fn read_pred<'b>(&'b mut self, pred: impl FnOnce(u8) -> bool) -> Option<u8> {
        if let Some(value) = self.peek() {
            if pred(value) {
                self.skip();
                return Some(value);
            }
        }
        None
    }

    pub fn skip(&mut self) {
        let _ = self.read();
    }

    pub fn skip_n(&mut self, n: usize) {
        let _ = self.read_n(n);
    }

    pub fn peek(&self) -> Option<u8> {
        self.peek_n(1).map(|slice| slice[0])
    }

    pub fn peek_at(&self, index: usize) -> Option<u8> {
        if self.buffer.len() <= index {
            return None;
        }
        Some(self.buffer[index])
    }

    pub fn peek_n<'b>(&'b self, n: usize) -> Option<&'a [u8]> {
        if self.buffer.len() < n {
            return None;
        }
        Some(&self.buffer[..n])
    }

    pub fn read_until<'b>(&'b mut self, value: u8) -> Option<&'a [u8]> {
        for i in 0..self.buffer.len() {
            if self.peek_at(i).unwrap() == value {
                let ret = self.read_n(i).unwrap();
                self.skip();
                return Some(ret);
            }
        }
        None
    }

    pub fn read_until_or_end<'b>(&'b mut self, value: u8) -> &'a [u8] {
        self.read_until(value)
            .unwrap_or_else(|| self.read_n(self.buffer.len()).unwrap())
    }

    pub fn at_end(&self) -> bool {
        self.buffer.len() == 0
    }

    pub fn read_amount_bytes(&self) -> usize {
        self.original.len() - self.buffer.len()
    }
}
