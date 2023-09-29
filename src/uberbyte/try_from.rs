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

macro_rules! try_from_signed {
    ($source:ty) => {
        impl TryFrom<$source> for UberByte {
            type Error = UberByteError;
        
            fn try_from(value: $source) -> Result<Self, Self::Error> {
                match value {
                    ..=0 => Err(UberByteError::ValueUnderflow),
                    _ => match u8::try_from(value) {
                        Ok(vu8) => Ok(UberByte::from(vu8)),
                        Err(_) => Err(UberByteError::ValueOverflow),
                    }
                }
            }
        }
        
        impl TryFrom<&$source> for UberByte {
            type Error = UberByteError;
        
            fn try_from(value: &$source) -> Result<Self, Self::Error> {
                let cloned_value = value.clone();
                return UberByte::try_from(cloned_value);
            }
        }
    };
}



try_from_unsigned!(u16);
try_from_unsigned!(u32);
try_from_unsigned!(u64);
try_from_unsigned!(u128);
try_from_unsigned!(usize);

try_from_signed!(i8);
try_from_signed!(i16);
try_from_signed!(i32);
try_from_signed!(i64);
try_from_signed!(i128);
try_from_signed!(isize);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_unsigned {
        ($name: ident, $source:ty) => {
            #[test]
            fn $name() {
                let max: $source = <$source>::MAX;
                let min: $source = <$source>::MIN;
                let valid: $source = 50;

                assert!(UberByte::try_from(max).is_err());
                assert!(UberByte::try_from(min).is_ok());

                let valid_result = UberByte::try_from(valid);
                assert!(valid_result.is_ok());
                assert_eq!(UberByte::from(50u8), valid_result.unwrap());
            }
        };
    }

    macro_rules! test_signed {
        ($name: ident, $source:ty) => {
            #[test]
            fn $name() {
                let min_value: $source = <$source>::MIN;
                let max_value: $source = <$source>::MAX;
                let upper: $source = 255;
                let valid: $source = 50;

                assert!(UberByte::try_from(min_value).is_err());
                assert!(UberByte::try_from(max_value).is_err());
                assert!(UberByte::try_from(upper).is_ok());

                // Blind unwrap here because we do not expect an error at this point
                // 50 is a valid value for any signed and unsigned number type
                let valid_byte = UberByte::try_from(valid).unwrap();

                assert_eq!(UberByte::from(50u8), valid_byte);
            }
        };
    }

    test_unsigned!(u16, u16);
    test_unsigned!(u32, u32);
    test_unsigned!(u64, u64);
    test_unsigned!(u128, u128);
    test_unsigned!(usize, usize);

    test_signed!(i16, i16);
    test_signed!(i32, i32);
    test_signed!(i64, i64);
    test_signed!(i128, i128);
    test_signed!(isize, isize);
    
}
