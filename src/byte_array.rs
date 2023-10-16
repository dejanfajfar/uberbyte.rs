use std::{ops::Index, ops::{IndexMut, AddAssign}, str::FromStr, io::Write};

use crate::{UberByte, UberByteError};

/// A simple implementation of a byte array composed of UberBytes
#[derive(Debug)]
pub struct ByteArray {
    data: Vec<UberByte>,
}

/// A simple abstract UberByte array
impl ByteArray {
    /// Adds a _UberByte_ to the ByteArray
    pub fn add_mut(&mut self, byte: UberByte) -> &mut Self {
        self.data.push(byte);
        return self;
    }

    pub fn add_range_mut(&mut self, bytes: Vec<UberByte>) -> &mut Self{
        for byte in bytes {
            self.add_mut(byte);
        }
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

    pub fn add_parity_byte(&mut self) {

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
            ret_val.add_mut(UberByte::from(byte));
        }

        return Ok(ret_val);
    }
}

impl Write for ByteArray {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let byte_array = ByteArray::from(buf);
        for byte in byte_array.into_iter() {
            self.add_mut(byte);
        }
        return Ok(buf.len());
    }

    fn flush(&mut self) -> std::io::Result<()> {
        return Ok(());
    }
}

impl AddAssign for ByteArray {
    fn add_assign(&mut self, rhs: Self) {
        for byte in rhs.into_iter() {
            self.add_mut(byte);
        }
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
    #[should_panic]
    fn indexer_out_of_range(){
        ByteArray::default()[usize::MAX];
    }

    #[test]
    fn get(){
        let vector = vec![1, 2, 3, 4, 5];
        let test_array: ByteArray = ByteArray::from(vector);

        assert_eq!(&UberByte::from(1), test_array.get(0).unwrap());
        assert_eq!(&UberByte::from(2), test_array.get(1).unwrap());
        assert_eq!(&UberByte::from(3), test_array.get(2).unwrap());
        assert_eq!(&UberByte::from(4), test_array.get(3).unwrap());
        assert_eq!(&UberByte::from(5), test_array.get(4).unwrap());
    }

    #[test]
    fn get_index_out_of_range(){
        assert!(ByteArray::default().get(usize::MAX).is_none());
    }

    #[test]
    fn len() {
        let mut test_array = ByteArray::default();
        
        assert_eq!(0, test_array.len());

        test_array.add_mut(UberByte::from(15));

        assert_eq!(1, test_array.len());
    }
}
