use super::{UberByte, UberByteError};

macro_rules! try_from_unsigned {
    ($source:ty) => {
        impl TryFrom<$source> for UberByte {
            type Error = UberByteError;

            #[inline]
            fn try_from(u: $source) -> Result<Self, Self::Error> {
                match u8::try_from(u) {
                    Ok(value_u8) => Ok(UberByte::from(value_u8)),
                    Err(_) => Err(UberByteError::ValueOverflow),
                }
            }
        }
    };
}

try_from_unsigned!(u16);
try_from_unsigned!(u32);
try_from_unsigned!(u64);
try_from_unsigned!(u128);
try_from_unsigned!(usize);


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_u16(){
        let test_value: u16 = 50;
        let test_result = UberByte::try_from(test_value);
        assert!(test_result.is_ok());
        assert_eq!(UberByte::from(50), test_result.unwrap());
    }

    #[test]
    fn from_u16_overflow(){
        let test_value: u16 = u16::MAX;
        let test_result = UberByte::try_from(test_value);
        assert!(test_result.is_err());
    }
}