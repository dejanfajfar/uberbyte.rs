use super::{UberByte, UberByteError};

impl From<u8> for UberByte {
    fn from(value: u8) -> Self {
        UberByte { value: value }
    }
}

impl From<&u8> for UberByte {
    fn from(value: &u8) -> Self {
        UberByte {
            value: value.clone(),
        }
    }
}

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

        impl TryFrom<&$source> for UberByte {
            type Error = UberByteError;

            #[inline]
            fn try_from(u: &$source) -> Result<Self, Self::Error> {
                match u8::try_from(u.clone()) {
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

    macro_rules! test_try_from {
        ($name:tt, $source:ty) => {
            #[test]
            fn $name() {
                let test_value: $source = 50;
                let test_result = UberByte::try_from(test_value);
                assert!(test_result.is_ok());
                assert_eq!(UberByte::from(50), test_result.unwrap());
            }
        };
    }

    macro_rules! test_try_from_overflow {
        ($name:tt, $source:ty) => {
            #[test]
            fn $name() {
                let test_value: $source = <$source>::MAX;
                let test_result = UberByte::try_from(test_value);
                assert!(test_result.is_err());
            }
        };
    }

    test_try_from!(try_from_u32, u32);
    test_try_from_overflow!(try_from_u32_overflow, u32);
    test_try_from_overflow!(try_from_u16_overflow, u16);
    test_try_from!(try_from_u16, u16);
    test_try_from!(try_from_u64, u64);
    test_try_from_overflow!(try_from_u64_overflow, u64);
    test_try_from!(try_from_u128, u128);
    test_try_from_overflow!(try_from_u128_overflow, u128);
    test_try_from!(try_from_usize, usize);
    test_try_from_overflow!(try_from_usize_overflow, usize);
}
