use std::{ops::Index, ops::IndexMut, str::FromStr, io::Write};

use crate::{UberByte, UberByteError};

/// A simple implementation of a byte array composed of UberBytes
/// 
/// # Remarks
/// 
/// 
#[derive(Debug)]
pub struct ByteArray {
    data: Vec<UberByte>,
}

/// Implements a simple byte array
impl ByteArray {
    /// Adds a _UberByte_ to the ByteArray
    pub fn add(&mut self, byte: UberByte) -> &mut Self {
        self.data.push(byte);
        return self;
    }

    pub fn get(&self, index: usize) -> Option<&UberByte> {
        self.data.get(index)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Simple alias for the __default__
    pub fn new() -> Self {
        ByteArray::default()
    }
}

impl Default for ByteArray {
    fn default() -> Self {
        ByteArray { data: vec![] }
    }
}

impl Index<usize> for ByteArray {
    type Output = UberByte;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for ByteArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl From<&[u8]> for ByteArray {
    fn from(value: &[u8]) -> Self {
        ByteArray {
            data: value.into_iter().map(|f: &u8| UberByte::from(f)).collect(),
        }
    }
}

impl From<Vec<u8>> for ByteArray {
    fn from(value: Vec<u8>) -> Self {
        ByteArray {
            data: value.into_iter().map(|f: u8| UberByte::from(f)).collect(),
        }
    }
}

impl IntoIterator for ByteArray {
    type Item = UberByte;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl FromStr for ByteArray {
    type Err = UberByteError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret_val = ByteArray::default();
        for byte in s.as_bytes() {
            ret_val.add(UberByte::from(byte));
        }

        return Ok(ret_val);
    }
}

impl Write for ByteArray {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn from_str() {
        let foo: Result<ByteArray, UberByteError> = ByteArray::from_str("tattoo");

        let byte_array: ByteArray = foo.unwrap();

        assert_eq!(6, byte_array.len());

        for byte in byte_array.into_iter() {
            println!("{:b}", byte);
        }
    }

    #[test]
    fn indexer() {
        let vector = vec![1, 2, 3, 4, 5];
        let test_array: ByteArray = ByteArray::from(vector);

        assert_eq!(UberByte::from(1), test_array[0]);
    }

    #[test]
    fn len() {
        let mut test_array = ByteArray::default();
        
        assert_eq!(0, test_array.len());

        test_array.add(UberByte::from(15));

        assert_eq!(1, test_array.len());
    }
}
