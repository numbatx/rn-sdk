
use crate::big_uint_mock::*;

use num_traits::sign::Signed;
use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

use alloc::vec::Vec;
use numbat_wasm::BigIntApi;

use num_bigint::BigInt;
use core::cmp::Ordering;

#[derive(Debug)]
pub struct RustBigInt(pub num_bigint::BigInt);

impl RustBigInt {
    pub fn value(&self) -> &BigInt {
        &self.0
    }
}

impl From<RustBigUint> for RustBigInt {
    fn from(item: RustBigUint) -> Self {
        RustBigInt(item.0)
    }
}

impl From<i64> for RustBigInt {
    fn from(item: i64) -> Self {
        RustBigInt(item.into())
    }
}

impl From<i32> for RustBigInt {
    fn from(item: i32) -> Self {
        RustBigInt(item.into())
    }
}

impl From<BigInt> for RustBigInt {
    fn from(item: BigInt) -> Self {
        RustBigInt(item)
    }
}

impl Clone for RustBigInt {
    fn clone(&self) -> Self {
        RustBigInt(self.0.clone())
    }
}

macro_rules! binary_operator {
    ($trait:ident, $method:ident) => {
        impl $trait for RustBigInt {
            type Output = RustBigInt;
        
            fn $method(self, other: RustBigInt) -> RustBigInt {
                RustBigInt((self.0).$method(other.0))
            }
        }

        impl<'a, 'b> $trait<&'b RustBigInt> for &'a RustBigInt {
            type Output = RustBigInt;
        
            fn $method(self, other: &RustBigInt) -> RustBigInt {
                RustBigInt(self.0.clone().$method(other.0.clone()))
            }
        }
    }
}

binary_operator!{Add, add}
binary_operator!{Sub, sub}
binary_operator!{Mul, mul}
binary_operator!{Div, div}
binary_operator!{Rem, rem}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident) => {
        impl $trait<RustBigInt> for RustBigInt {
            fn $method(&mut self, other: Self) {
                BigInt::$method(&mut self.0, other.0)
            }
        }
        
        impl $trait<&RustBigInt> for RustBigInt {
            fn $method(&mut self, other: &RustBigInt) {
                BigInt::$method(&mut self.0, &other.0)
            }
        }
    }
}

binary_assign_operator!{AddAssign, add_assign}
binary_assign_operator!{SubAssign, sub_assign}
binary_assign_operator!{MulAssign, mul_assign}
binary_assign_operator!{DivAssign, div_assign}
binary_assign_operator!{RemAssign, rem_assign}

impl Neg for RustBigInt {
    type Output = RustBigInt;

    fn neg(self) -> Self::Output {
        RustBigInt(-self.0)
    }
}

impl PartialEq<Self> for RustBigInt {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }
}

impl Eq for RustBigInt{}

impl PartialOrd<Self> for RustBigInt {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl Ord for RustBigInt {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.0, &other.0)
    }
}

impl PartialEq<i64> for RustBigInt {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        PartialEq::eq(&self.0, &BigInt::from(*other))
    }
}

impl PartialOrd<i64> for RustBigInt {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &BigInt::from(*other))
    }
}

use numbat_wasm::numbat_codec::*;

impl Encode for RustBigInt {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
        let bytes = self.to_signed_bytes_be();
        f(&bytes);
        Ok(())
    }
    
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        let bytes = self.to_signed_bytes_be();
        bytes.as_slice().dep_encode_to(dest)
    }
}

impl Decode for RustBigInt {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let bytes = input.flush()?;
        Ok(RustBigInt::from_signed_bytes_be(bytes))
    }

    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let size = usize::dep_decode(input)?;
        let bytes = input.read_slice(size)?;
        Ok(RustBigInt::from_signed_bytes_be(bytes))
    }
}

impl numbat_wasm::BigIntApi<RustBigUint> for RustBigInt {
    
    fn abs_uint(&self) -> RustBigUint {
        RustBigUint(self.0.abs())
    }

    fn sign(&self) -> numbat_wasm::Sign {
        match self.0.sign() {
            num_bigint::Sign::Minus => numbat_wasm::Sign::NoSign,
            num_bigint::Sign::NoSign => numbat_wasm::Sign::NoSign,
            num_bigint::Sign::Plus => numbat_wasm::Sign::Plus,
        }
    }

    fn to_signed_bytes_be(&self) -> Vec<u8> {
        self.0.to_signed_bytes_be()
    }

    fn from_signed_bytes_be(bytes: &[u8]) -> Self {
        let bi = BigInt::from_signed_bytes_be(bytes);
        bi.into()
    }
}

impl RustBigInt {
    pub fn to_signed_bytes_be(&self) -> Vec<u8>{
        self.0.to_signed_bytes_be()
    }
}
