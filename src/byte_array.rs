use std::str::FromStr;

use crate::{UberByte, UberByteError};

#[derive(Debug)]
pub struct ByteArray{
    data: Vec<UberByte>
}

/// Implements a simple byte array
impl ByteArray{
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
}

impl Default for ByteArray{
    fn default() -> Self {
        ByteArray { data: vec![] }
    }
}

impl From<&[u8]> for ByteArray {
    fn from(value: &[u8]) -> Self {
        ByteArray {
            data: value.into_iter().map(|f| UberByte::from(f)).collect()
        }
    }
}

impl IntoIterator for ByteArray{
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
        for byte in s.as_bytes(){
            ret_val.add(UberByte::from(byte));
        }

        return Ok(ret_val);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn foo_test(){
        let foo = ByteArray::from_str("tattoo");

        let byte_array = foo.unwrap();

        for byte in byte_array.into_iter() {
            println!("{:b}", byte);
        }
    }
}