#[doc(hidden)]
mod __position {
    #[repr(transparent)]
    #[derive(Copy)]
    #[derive(Clone)]
    #[doc = " A position: differences are plain `f32`, and positions cannot be added."]
    pub struct Position(f32);
    
    #[allow(unused_imports)]
    use super:: * ;
    #[allow(dead_code)]
    #[allow(unused_qualifications)]
    #[allow(clippy::unnecessary_cast)]
    #[allow(clippy::manual_range_contains)]
    const _: () = {
        if!(size_of:: <Position>()== size_of:: <f32>()){
            core::panic!("invalid integral storage size");
        };
        if!(align_of:: <Position>()== align_of:: <f32>()){
            core::panic!("invalid integral storage alignment");
        };
        trait Seal {
            fn conv_position(self) -> f32;
        }trait Make: Seal {
            fn make(self) -> Position;
        }trait Math: Seal {}trait Rel: Seal {}impl Seal for Position {
            #[inline(always)]
            fn conv_position(self) -> f32 {
                return Self::raw(self);
            }
        }
        impl Make for Position {
            #[inline(always)]
            fn make(self) -> Position {
                let raw =  <Self as Seal> ::conv_position(self);
                return Position::_or_panic(raw);
            }
        }
        impl Rel for Position {}
        impl Seal for f32 {
            #[inline(always)]
            fn conv_position(self) -> f32 {
                return self as f32;
            }
        }
        impl Make for f32 {
            #[inline(always)]
            fn make(self) -> Position {
                let raw =  <Self as Seal> ::conv_position(self);
                return Position::_or_panic(raw);
            }
        }
        impl Seal for i16 {
            #[inline(always)]
            fn conv_position(self) -> f32 {
                return self as f32;
            }
        }
        impl Make for i16 {
            #[inline(always)]
            fn make(self) -> Position {
                let raw =  <Self as Seal> ::conv_position(self);
                return Position::_or_panic(raw);
            }
        }
        impl Seal for i8 {
            #[inline(always)]
            fn conv_position(self) -> f32 {
                return self as f32;
            }
        }
        impl Make for i8 {
            #[inline(always)]
            fn make(self) -> Position {
                let raw =  <Self as Seal> ::conv_position(self);
                return Position::_or_panic(raw);
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl Seal for isize {
            #[inline(always)]
            fn conv_position(self) -> f32 {
                return self as f32;
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl Make for isize {
            #[inline(always)]
            fn make(self) -> Position {
                let raw =  <Self as Seal> ::conv_position(self);
                return Position::_or_panic(raw);
            }
        }
        impl Seal for u16 {
            #[inline(always)]
            fn conv_position(self) -> f32 {
                return self as f32;
            }
        }
        impl Make for u16 {
            #[inline(always)]
            fn make(self) -> Position {
                let raw =  <Self as Seal> ::conv_position(self);
                return Position::_or_panic(raw);
            }
        }
        impl Seal for u8 {
            #[inline(always)]
            fn conv_position(self) -> f32 {
                return self as f32;
            }
        }
        impl Make for u8 {
            #[inline(always)]
            fn make(self) -> Position {
                let raw =  <Self as Seal> ::conv_position(self);
                return Position::_or_panic(raw);
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl Seal for usize {
            #[inline(always)]
            fn conv_position(self) -> f32 {
                return self as f32;
            }
        }
        #[cfg(target_pointer_width = "16")]
        impl Make for usize {
            #[inline(always)]
            fn make(self) -> Position {
                let raw =  <Self as Seal> ::conv_position(self);
                return Position::_or_panic(raw);
            }
        }
        impl Position {
            #[must_use]
            #[inline(always)]
            #[allow(private_bounds)]
            pub fn of<T>(it: T,) -> Position where T: Make, {
                return<T as Make> ::make(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn make(it: f32) -> Position {
                return Self::_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            pub const fn raw(self) -> f32 {
                return self.0;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_f32(self) -> f32 {
                let it = Self::raw(self);
                return it as f32;
            }
            #[must_use]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn into_f64(self) -> f64 {
                let it = Self::raw(self);
                return it as f64;
            }
            #[inline(always)]
            pub fn try_map<F>(self, function: F,) ->  Result<Self, f32>where F: FnOnce(f32) -> f32, {
                let value = function(Self::raw(self));
                return 
                    Ok(Self(value))
                ;
            }
            #[must_use]
            #[inline(always)]
            pub fn map<F>(self, function: F,) -> Self where F: FnOnce(f32) -> f32, {
                return match self.try_map(function){
                    Ok(value) => value, 
                    Err(_) => {
                        core::panic!("mapping operation produced an invalid value")
                    }
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_f32(self) ->  Result<f32, ()>{
                return Ok(Self::raw(self)as f32);
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_f64(self) ->  Result<f64, ()>{
                return Ok(Self::raw(self)as f64);
            }
            #[cfg(target_pointer_width = "16")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) ->  Result<usize, ()>{
                let r = Self::raw(self);
                let t = r as usize;
                return if t as f32==r {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[cfg(target_pointer_width = "32")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) ->  Result<usize, ()>{
                let r = Self::raw(self);
                let t = r as usize;
                return if t as f32==r&&r<4294967296.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[cfg(target_pointer_width = "64")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_usize(self) ->  Result<usize, ()>{
                let r = Self::raw(self);
                let t = r as usize;
                return if t as f32==r&&r<18446744073709551616.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[cfg(target_pointer_width = "16")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) ->  Result<isize, ()>{
                let r = Self::raw(self);
                let t = r as isize;
                return if t as f32==r {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[cfg(target_pointer_width = "32")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) ->  Result<isize, ()>{
                let r = Self::raw(self);
                let t = r as isize;
                return if t as f32==r&&r<2147483648.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[cfg(target_pointer_width = "64")]
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_isize(self) ->  Result<isize, ()>{
                let r = Self::raw(self);
                let t = r as isize;
                return if t as f32==r&&r<9223372036854775808.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u8(self) ->  Result<u8, ()>{
                let r = Self::raw(self);
                let t = r as u8;
                return if t as f32==r {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u16(self) ->  Result<u16, ()>{
                let r = Self::raw(self);
                let t = r as u16;
                return if t as f32==r {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u32(self) ->  Result<u32, ()>{
                let r = Self::raw(self);
                let t = r as u32;
                return if t as f32==r&&r<4294967296.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u64(self) ->  Result<u64, ()>{
                let r = Self::raw(self);
                let t = r as u64;
                return if t as f32==r&&r<18446744073709551616.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_u128(self) ->  Result<u128, ()>{
                let r = Self::raw(self);
                let t = r as u128;
                return if t as f32==r&&r< f32::INFINITY {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i8(self) ->  Result<i8, ()>{
                let r = Self::raw(self);
                let t = r as i8;
                return if t as f32==r {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i16(self) ->  Result<i16, ()>{
                let r = Self::raw(self);
                let t = r as i16;
                return if t as f32==r {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i32(self) ->  Result<i32, ()>{
                let r = Self::raw(self);
                let t = r as i32;
                return if t as f32==r&&r<2147483648.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i64(self) ->  Result<i64, ()>{
                let r = Self::raw(self);
                let t = r as i64;
                return if t as f32==r&&r<9223372036854775808.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[inline(always)]
            #[allow(clippy::unnecessary_cast)]
            pub const fn try_into_i128(self) ->  Result<i128, ()>{
                let r = Self::raw(self);
                let t = r as i128;
                return if t as f32==r&&r<170141183460469231731687303715884105728.0 {
                    Ok(t)
                }else {
                    Err(())
                };
            }
            #[must_use]
            #[inline(always)]
            pub(self)const fn _sub(&self, rhs: f32,) -> Self {
                let lhs = Self::raw(*self);
                let it = lhs-rhs;
                return Self::_or_panic(it);
            }
            #[must_use]
            #[inline(always)]
            #[doc(hidden)]
            pub(self)const fn _eq(self, rhs: f32,) -> bool {
                let lhs = Self::raw(self);
                return lhs==rhs;
            }
            #[inline(always)]
            pub const fn try_make(it: f32) -> Result<Self, f32>{
                return Ok(Self::_or_panic(it));
            }
            #[must_use]
            #[inline(always)]
            pub(self)const fn _or_panic(it: f32) -> Self {
                return Self(it);
            }
        }
        #[allow(clippy::unnecessary_cast)]
        impl Position {}
        impl AsRef<f32>for Position {
            #[inline(always)]
            fn as_ref(&self) ->  &f32 {
                return&self.0;
            }
        }
        impl core::str::FromStr for Position {
            type Err = ();
            #[inline(always)]
            fn from_str(source: &str,) ->  Result<Self, Self::Err>{
                let value = match <f32 as core::str::FromStr> ::from_str(source){
                    Ok(value) => value, 
                    Err(_) => {
                        return Err(());
                    }
                };
                return 
                    Ok(Self(value))
                ;
            }
        }
        impl Into<f32>for Position {
            #[inline(always)]
            fn into(self) -> f32 {
                return Self::into_f32(self);
            }
        }
        impl Into<f64>for Position {
            #[inline(always)]
            fn into(self) -> f64 {
                return Self::into_f64(self);
            }
        }
        impl TryInto<usize>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<usize, Self::Error>{
                return Self::try_into_usize(self);
            }
        }
        impl TryInto<isize>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<isize, Self::Error>{
                return Self::try_into_isize(self);
            }
        }
        impl TryInto<u8>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<u8, Self::Error>{
                return Self::try_into_u8(self);
            }
        }
        impl TryInto<u16>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<u16, Self::Error>{
                return Self::try_into_u16(self);
            }
        }
        impl TryInto<u32>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<u32, Self::Error>{
                return Self::try_into_u32(self);
            }
        }
        impl TryInto<u64>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<u64, Self::Error>{
                return Self::try_into_u64(self);
            }
        }
        impl TryInto<u128>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<u128, Self::Error>{
                return Self::try_into_u128(self);
            }
        }
        impl TryInto<i8>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<i8, Self::Error>{
                return Self::try_into_i8(self);
            }
        }
        impl TryInto<i16>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<i16, Self::Error>{
                return Self::try_into_i16(self);
            }
        }
        impl TryInto<i32>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<i32, Self::Error>{
                return Self::try_into_i32(self);
            }
        }
        impl TryInto<i64>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<i64, Self::Error>{
                return Self::try_into_i64(self);
            }
        }
        impl TryInto<i128>for Position {
            type Error = ();
            #[inline(always)]
            fn try_into(self,) ->  Result<i128, Self::Error>{
                return Self::try_into_i128(self);
            }
        }
        impl core::fmt::Debug for Position {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_> ,) ->  core::fmt::Result {
                return f.debug_tuple("Position").field(&self.0).finish();
            }
        }
        impl core::ops::Sub for Position {
            type Output = f32;
            #[inline(always)]
            fn sub(self, rhs: Self,) -> Self::Output {
                return Position::into_offset(self._sub(rhs.0));
            }
        }
        impl <T> core::ops::Sub<T>for Position where T: Math+Seal {
            type Output = f32;
            #[inline(always)]
            fn sub(self, rhs: T,) -> Self::Output {
                let it = Seal::conv_position(rhs);
                return Position::into_offset(self._sub(it));
            }
        }
        impl <T> PartialEq<T>for Position where T: Rel+Seal+ Copy {
            #[inline(always)]
            fn eq(&self, rhs: &T,) -> bool {
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_position(*rhs);
                return lhs==rhs;
            }
        }
        impl <T> PartialOrd<T>for Position where T: Rel+Seal+ Copy {
            #[inline(always)]
            fn partial_cmp(&self, rhs: &T,) ->  Option< core::cmp::Ordering>{
                let lhs = Self::raw(*self);
                let rhs = Seal::conv_position(*rhs);
                return PartialOrd::partial_cmp(&lhs, &rhs);
            }
            #[inline(always)]
            fn lt(&self, rhs: &T,) -> bool {
                return Self::raw(*self)<Seal::conv_position(*rhs);
            }
            #[inline(always)]
            fn le(&self, rhs: &T,) -> bool {
                return Self::raw(*self)<=Seal::conv_position(*rhs);
            }
            #[inline(always)]
            fn gt(&self, rhs: &T,) -> bool {
                return Self::raw(*self)>Seal::conv_position(*rhs);
            }
            #[inline(always)]
            fn ge(&self, rhs: &T,) -> bool {
                return Self::raw(*self)>=Seal::conv_position(*rhs);
            }
        }
        impl core::fmt::LowerExp for Position {
            #[inline(always)]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_> ,) ->  core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::LowerExp::fmt(&raw, f);
            }
        }
        impl core::fmt::UpperExp for Position {
            #[inline(always)]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_> ,) ->  core::fmt::Result {
                let raw = Self::raw(*self);
                return core::fmt::UpperExp::fmt(&raw, f);
            }
        }
    };
}
#[allow(unused_imports)]
pub use __position::Position;

impl Position {
    fn into_offset(self) -> f32 {
        return self.raw();
    }
}

fn main() {
    let start = Position::make(1.5);
    let end = Position::make(4.0);

    let offset: f32 = end - start;
    assert_eq!(offset, 2.5);
    assert!(start < end);

    println!("{offset}");
}
