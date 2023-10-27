use std::{
    io::Write,
    ops::Index,
    ops::{AddAssign, IndexMut},
    slice::SliceIndex,
    str::FromStr,
};

use crate::{UberByte, UberByteError};

/// A simple implementation of a byte array composed of UberBytes
#[derive(Debug, Clone)]
pub struct ByteArray {
    data: Vec<UberByte>,
}

/// A simple abstract UberByte array
impl ByteArray {
    /// Add the byte to the byte array
    ///
    /// # Returns
    ///
    /// A copy of the original array with the new byte attached
    pub fn add(&self, byte: UberByte) -> Self {
        let mut clone = self.clone();
        clone.add_mut(byte);
        return clone;
    }

    /// Adds a _UberByte_ to the ByteArray
    pub fn add_mut(&mut self, byte: UberByte) {
        self.data.push(byte);
    }

    /// Adds a range of UberBytes to the existing ByteArray
    ///
    /// # Returns
    ///
    /// The byte array with the byte added
    pub fn add_range_mut(&mut self, bytes: Vec<UberByte>) {
        for byte in bytes {
            self.add_mut(byte);
        }
    }

    /// Adds many bytes to the given byte array
    ///
    /// # Returns
    ///
    /// Returns a clone of the original array with the bytes attached
    pub fn add_range(&self, bytes: Vec<UberByte>) -> Self {
        let mut clone = self.clone();
        clone.add_range_mut(bytes);
        return clone;
    }

    /// Retrieves the byte at the specific index
    ///
    /// # Returns
    ///
    /// A option of a UberByte
    ///
    /// If a byte exists at this index then a copy of this Byte will be returned
    /// If not the None will be returned
    pub fn get(&self, index: usize) -> Option<&UberByte> {
        self.data.get(index)
    }

    /// Returns the number of bytes stored in this byte array
    ///
    /// # Returns
    ///
    /// The number of bytes stored in the byte array
    ///
    /// # Remarks
    ///
    /// If the byte array is empty then 0 will be returned
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Simple alias for the __default__
    pub fn new() -> Self {
        ByteArray::default()
    }

    /// Adds a simple parity byte so that the complete number ob bits is dividable by 2
    ///
    /// # Returns
    ///
    /// Returns a indicator that a parity _byte_ was added.
    ///
    /// # Remarks
    ///
    /// If a parity byte was added then the length of the array was changed and will be longer by 1
    pub fn add_parity_byte(&mut self) -> bool {
        let sum_of_bits: u8 = self.clone().into_iter().map(|b| b.count_set_bits()).sum();

        if sum_of_bits % 2 == 0 {
            return false;
        }

        self.add_mut(UberByte::from(0b_0000_0001));
        return true;
    }
}

impl Default for ByteArray {
    fn default() -> Self {
        ByteArray { data: vec![] }
    }
}

impl IndexMut<usize> for ByteArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<Idx> Index<Idx> for ByteArray
where
    Idx: SliceIndex<[UberByte]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index]
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
    }

    #[test]
    fn indexer() {
        let vector = vec![1, 2, 3, 4, 5];
        let test_array: ByteArray = ByteArray::from(vector);

        assert_eq!(UberByte::from(1), test_array[0]);
    }

    #[test]
    #[should_panic]
    fn indexer_out_of_range() {
        ByteArray::default()[usize::MAX];
    }

    #[test]
    fn get() {
        let vector = vec![1, 2, 3, 4, 5];
        let test_array: ByteArray = ByteArray::from(vector);

        assert_eq!(&UberByte::from(1), test_array.get(0).unwrap());
        assert_eq!(&UberByte::from(2), test_array.get(1).unwrap());
        assert_eq!(&UberByte::from(3), test_array.get(2).unwrap());
        assert_eq!(&UberByte::from(4), test_array.get(3).unwrap());
        assert_eq!(&UberByte::from(5), test_array.get(4).unwrap());
    }

    #[test]
    fn get_index_out_of_range() {
        assert!(ByteArray::default().get(usize::MAX).is_none());
    }

    #[test]
    fn len() {
        let mut test_array = ByteArray::default();

        assert_eq!(0, test_array.len());

        test_array.add_mut(UberByte::from(15));

        assert_eq!(1, test_array.len());
    }

    #[test]
    fn add() {
        let test_array = ByteArray::default();

        let new_test_array = test_array.add(UberByte::from(12));

        assert_eq!(0, test_array.len());
        assert_eq!(1, new_test_array.len());
    }

    #[test]
    fn add_mut() {
        let mut test_array = ByteArray::default();

        test_array.add_mut(UberByte::from(12));

        assert_eq!(1, test_array.len());
    }

    #[test]
    fn add_range() {
        let test_array = ByteArray::default();

        let result_array = test_array.add_range(vec![UberByte::from(12), UberByte::from(13)]);

        assert_eq!(2, result_array.len());
        assert_eq!(0, test_array.len());
    }

    #[test]
    fn add_range_mut() {
        let mut test_array = ByteArray::default();

        test_array.add_range_mut(vec![UberByte::from(12), UberByte::from(13)]);

        assert_eq!(2, test_array.len());
    }
}
