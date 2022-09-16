//! # uX - non-standard-width integers types
//!
//! When non-standard-width integers is required in an applications, the norm is to use a larger container and make sure the value is within range after manipulation. uX aims to take care of this once and for all by:
//!
//! - Providing `u1`-`u127` and `i1`-`i127` types that should behave as similar as possible to the built in rust types
//!     - The methods of the defined types are the same as for the built in types (far from all is implemented at this point but fill out an issue or create a PR if something essential for you is missing)
//!     - Overflow will panic in debug and wrap in release.
//! - All possible infallible conversions is possible by using `From` and all fallible conversion by using `TryFrom`.
//!
//! The uX types take up as much space as the smallest integer type that can contain them;
//! the compiler can not yet be made aware of further optimization potential,
//! and thus does not use it:
//! an `Option<u7>` still takes up two bytes.

#![no_std]

mod conversion;
mod niche;

use core::cmp::{Ord, Ordering, PartialOrd};
use core::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, UpperHex};
use core::hash::{Hash, Hasher};
use core::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign, Sub,
};
use core::{fmt, mem};

macro_rules! define_unsigned {
    ($(($doc:literal $name:ident $bits:literal $type:ident))*) => {
        $(
            impl $name {
                #[inline]
                fn mask(value: $type) -> Self {
                    let value = value & (((1 as $type) << $bits).overflowing_sub(1).0);
                    unsafe { $name::new_unchecked(value) }
                }

                #[inline]
                fn mask_pos(value: $type) -> Self {
                    let value = value & (((1 as $type) << $bits).overflowing_sub(1).0);
                    unsafe { $name::new_unchecked(value) }
                }

                #[inline]
                fn mask_neg(value: $type) -> Self {
                    let value = value & (((1 as $type) << $bits).overflowing_sub(1).0);
                    unsafe { $name::new_unchecked(value) }
                }
            }
        )*

        implement_common!($(($doc $name $bits $type))*);
        implement_layout_test!(test_unsigned_layout $($name $type)*);
    };
}

macro_rules! define_signed {
    ($(($doc:literal $name:ident $bits:literal $type:ident))*) => {
        $(
            impl $name {
                #[inline]
                fn mask(value: $type) -> Self {
                    let value = if value & (1 << ($bits - 1)) == 0 {
                        value & (((1 as $type) << $bits).overflowing_sub(1).0)
                    } else {
                        value | !(((1 as $type) << $bits).overflowing_sub(1).0)
                    };
                    unsafe { $name::new_unchecked(value) }
                }

                #[inline]
                fn mask_pos(value: $type) -> Self {
                    let value = value & (((1 as $type) << $bits).overflowing_sub(1).0);
                    unsafe { $name::new_unchecked(value) }
                }

                #[inline]
                fn mask_neg(value: $type) -> Self {
                    let value = value | !(((1 as $type) << $bits).overflowing_sub(1).0);
                    unsafe { $name::new_unchecked(value) }
                }
            }
        )*

        implement_common!($(($doc $name $bits $type))*);
        implement_layout_test!(test_signed_layout $($name $type)*);
    };
}

macro_rules! implement_common {
    ($(($doc:literal $name:ident $bits:literal $type:ident))*) => { $(
        #[doc = $doc]
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy)]
        pub struct $name(crate::niche::$name);

        impl $name {
            // The largest value that can be represented by this integer type.
            pub const MAX: Self = $name::new(crate::niche::$name::MAX);

            // The largest value that can be represented by this integer type.
            pub const MIN: Self = $name::new(crate::niche::$name::MIN);

            /// Returns the smallest value that can be represented by this integer type.
            #[inline]
            pub fn min_value() -> $name {
                $name::MIN
            }

            /// Returns the largest value that can be represented by this integer type.
            #[inline]
            pub fn max_value() -> $name {
                $name::MAX
            }

            /// Crates a new variable
            ///
            /// This function mainly exists as there is currently not a better way to construct these types.
            /// May be deprecated or removed if a better way to construct these types becomes available.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use ux::*;
            ///
            /// assert_eq!(u31::new(64), u31::from(64u8));
            ///
            /// ```
            ///
            /// # Panic
            ///
            /// This function will panic if `value` is not representable by this type
            #[allow(unreachable_patterns)]
            #[inline]
            pub const fn new(value: $type) -> $name {
                const MIN: $type = crate::niche::$name::MIN;
                const MAX: $type = crate::niche::$name::MAX;
                match value {
                    MIN..=MAX => unsafe { $name::new_unchecked(value) },
                    _ => panic!("value out of range")
                }
            }

            /// Try to crate a new variable
            ///
            /// If the value is out of range, [`None`] is returned.
            ///
            /// ```
            /// use ux::*;
            ///
            /// assert_eq!(i15::try_new(0x3fff).unwrap(), i15::new(0x3fff));
            /// assert_eq!(i15::try_new(0x4000), None);
            /// ```
            #[allow(unreachable_patterns)]
            #[inline]
            pub const fn try_new(value: core::primitive::$type) -> Option<$name> {
                const MIN: core::primitive::$type = crate::niche::$name::MIN;
                const MAX: core::primitive::$type = crate::niche::$name::MAX;
                match value {
                    MIN..=MAX => Some(unsafe { $name::new_unchecked(value) }),
                    _ => None,
                }
            }

            /// Crate a new variable without checking if it is in range
            ///
            /// Only use this function if you are sure that value is in range,
            /// or subtle, hard to debug runtime errors will occur!
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use ux::*;
            ///
            /// assert_eq!(unsafe { u31::new_unchecked(64) }, u31::new(64));
            /// ```
            #[inline]
            pub const unsafe fn new_unchecked(value: $type) -> $name {
                Self(mem::transmute(value))
            }

            /// Get the contained value
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use ux::*;
            ///
            /// assert_eq!(u31::new(64).get(), 64);
            /// ```
            #[inline]
            pub const fn get(self) -> $type {
                unsafe { mem::transmute(self.0) }
            }

            /// Wrapping (modular) subtraction. Computes `self - other`,
            /// wrapping around at the boundary of the type.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use ux::*;
            ///
            /// assert_eq!(i5::MIN.wrapping_sub(i5::new(1)), i5::MAX);
            ///
            /// assert_eq!(i5::new(-10).wrapping_sub(i5::new(5)), i5::new(-15));
            /// assert_eq!(i5::new(-15).wrapping_sub(i5::new(5)), i5::new(12));
            /// ```
            #[inline]
            pub fn wrapping_sub(self, rhs: Self) -> Self {
                $name::mask(self.get().wrapping_sub(rhs.get()))
            }

            /// Wrapping (modular) addition. Computes `self + other`,
            /// wrapping around at the boundary of the type.
            ///
            /// # Examples
            ///
            /// Basic usage:
            ///
            /// ```
            /// use ux::*;
            ///
            /// assert_eq!(i5::MAX.wrapping_add(i5::new(1)), i5::MIN);
            ///
            /// assert_eq!(i5::new(10).wrapping_add(i5::new(5)), i5::new(15));
            /// assert_eq!(i5::new(15).wrapping_add(i5::new(5)), i5::new(-12));
            /// ```
            #[inline]
            pub fn wrapping_add(self, rhs: Self) -> Self {
                $name::mask(self.get().wrapping_add(rhs.get()))
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                unsafe { $name::new_unchecked(0) }
            }
        }

        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.get() == other.get()
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &$name) -> Option<Ordering> {
                self.get().partial_cmp(&other.get())
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &$name) -> Ordering {
                self.get().cmp(&other.get())
            }
        }

        impl Hash for $name {
            #[inline]
            fn hash<H: Hasher>(&self, h: &mut H) {
                self.get().hash(h)
            }
        }

        // Implement formating functions
        impl Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                Debug::fmt(&self.get(), f)
            }
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                Display::fmt(&self.get(), f)
            }
        }

        impl UpperHex for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                UpperHex::fmt(&self.get(), f)
            }
        }

        impl LowerHex for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                LowerHex::fmt(&self.get(), f)
            }
        }

        impl Octal for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                Octal::fmt(&self.get(), f)
            }
        }

        impl Binary for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                Binary::fmt(&self.get(), f)
            }
        }

        impl<T> Shr<T> for $name
        where
            $type: Shr<T, Output = $type>,
        {
            type Output = $name;

            #[inline]
            #[allow(unused_comparisons)]
            fn shr(self, rhs: T) -> $name {
                let value = self.get();
                if value >= 0 {
                    $name::mask_pos(value.shr(rhs))
                } else {
                    $name::mask_neg(value.shr(rhs))
                }
            }
        }

        impl<T> Shl<T> for $name
        where
            $type: Shl<T, Output = $type>,
        {
            type Output = $name;

            #[inline]
            fn shl(self, rhs: T) -> $name {
                $name::mask(self.get().shl(rhs))
            }
        }

        impl<T> ShrAssign<T> for $name
        where
            $type: Shr<T, Output = $type>,
        {
            #[inline]
            fn shr_assign(&mut self, rhs: T) {
                *self = self.shr(rhs);
            }
        }

        impl<T> ShlAssign<T> for $name
        where
            $type: Shl<T, Output = $type>,
        {
            #[inline]
            fn shl_assign(&mut self, rhs: T) {
                *self = self.shl(rhs);
            }
        }

        impl BitOr<$name> for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, rhs: $name) -> Self::Output {
                $name::mask(self.get().bitor(rhs.get()))
            }
        }

        impl<'a> BitOr<&'a $name> for $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitor(self, rhs: &'a $name) -> Self::Output {
                $name::mask(self.get().bitor(rhs.get()))
            }
        }

        impl<'a> BitOr<$name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitor(self, rhs: $name) -> Self::Output {
                $name::mask(self.get().bitor(rhs.get()))
            }
        }

        impl<'a> BitOr<&'a $name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitor(self, rhs: &'a $name) -> Self::Output {
                $name::mask(self.get().bitor(rhs.get()))
            }
        }

        impl BitOrAssign<$name> for $name {
            #[inline]
            fn bitor_assign(&mut self, other: $name) {
                *self = self.bitor(other);
            }
        }

        impl BitXor<$name> for $name {
            type Output = $name;

            #[inline]
            fn bitxor(self, rhs: $name) -> Self::Output {
                $name::mask(self.get().bitxor(rhs.get()))
            }
        }

        impl<'a> BitXor<&'a $name> for $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitxor(self, rhs: &'a $name) -> Self::Output {
                $name::mask(self.get().bitxor(rhs.get()))
            }
        }

        impl<'a> BitXor<$name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitxor(self, rhs: $name) -> Self::Output {
                $name::mask(self.get().bitxor(rhs.get()))
            }
        }

        impl<'a> BitXor<&'a $name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitxor(self, rhs: &'a $name) -> Self::Output {
                $name::mask(self.get().bitxor(rhs.get()))
            }
        }

        impl BitXorAssign<$name> for $name {
            #[inline]
            fn bitxor_assign(&mut self, other: $name) {
                *self = self.bitxor(other);
            }
        }

        impl Not for $name {
            type Output = $name;

            #[inline]
            fn not(self) -> $name {
                $name::mask(self.get().not())
            }
        }

        impl<'a> Not for &'a $name {
            type Output = <$name as Not>::Output;

            #[inline]
            fn not(self) -> $name {
                $name::mask(self.get().not())
            }
        }

        impl BitAnd<$name> for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, rhs: $name) -> Self::Output {
                $name::mask(self.get().bitand(rhs.get()))
            }
        }

        impl<'a> BitAnd<&'a $name> for $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitand(self, rhs: &'a $name) -> Self::Output {
                $name::mask(self.get().bitand(rhs.get()))
            }
        }

        impl<'a> BitAnd<$name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitand(self, rhs: $name) -> Self::Output {
                $name::mask(self.get().bitand(rhs.get()))
            }
        }

        impl<'a> BitAnd<&'a $name> for &'a $name {
            type Output = <$name as BitOr<$name>>::Output;

            #[inline]
            fn bitand(self, rhs: &'a $name) -> Self::Output {
                $name::mask(self.get().bitand(rhs.get()))
            }
        }

        impl BitAndAssign<$name> for $name {
            #[inline]
            fn bitand_assign(&mut self, other: $name) {
                *self = self.bitand(other);
            }
        }

        impl Add<$name> for $name {
            type Output = $name;

            #[allow(unused_comparisons)]
            #[inline]
            fn add(self, other: $name) -> $name {
                if self.get() > 0 && other.get() > 0 {
                    debug_assert!($name::MAX.get() - other.get() >= self.get());
                } else if self.get() < 0 && other.get() < 0 {
                    debug_assert!($name::MIN.get() - other.get() <= self.get());
                }
                self.wrapping_add(other)
            }
        }

        impl Sub<$name> for $name {
            type Output = $name;

            #[allow(unused_comparisons)]
            #[inline]
            fn sub(self, other: $name) -> $name {
                if self > other {
                    debug_assert!($name::MAX.get() + other.get() >= self.get());
                } else if self < other {
                    debug_assert!($name::MIN.get() + other.get() <= self.get());
                }
                self.wrapping_sub(other)
            }
        }
    )* };
}

#[cfg(not(test))]
macro_rules! implement_layout_test {
    ($($_:tt)*) => {};
}

#[cfg(test)]
macro_rules! implement_layout_test {
    ($test_name:ident $($name:ident $type:ident)*) => {
        #[test]
        fn $test_name() {
            $(
                assert_eq!(
                    core::mem::size_of::<$name>(),
                    core::mem::size_of::<$type>(),
                    "size_of::<{}>() == size_of::<{}>()",
                    stringify!($name),
                    stringify!($type),
                );
                assert_eq!(
                    core::mem::align_of::<$name>(),
                    core::mem::align_of::<$type>(),
                    "align_of::<{}>() == align_of::<{}>()",
                    stringify!($name),
                    stringify!($type),
                );
                assert_eq!(
                    core::mem::size_of::<$name>(),
                    core::mem::size_of::<Option<$name>>(),
                    "size_of::<{}>() == size_of::<Option<{}>>()",
                    stringify!($name),
                    stringify!($name),
                );
                assert_eq!(
                    core::mem::align_of::<$name>(),
                    core::mem::align_of::<Option<$name>>(),
                    "align_of::<{}>() == align_of::<Option<{}>>()",
                    stringify!($name),
                    stringify!($name),
                );
            )*
        }
    };
}

define_unsigned! {
    ("The 1-bit unsigned integer type." u1 1 u8)
    ("The 2-bit unsigned integer type." u2 2 u8)
    ("The 3-bit unsigned integer type." u3 3 u8)
    ("The 4-bit unsigned integer type." u4 4 u8)
    ("The 5-bit unsigned integer type." u5 5 u8)
    ("The 6-bit unsigned integer type." u6 6 u8)
    ("The 7-bit unsigned integer type." u7 7 u8)
    ("The 9-bit unsigned integer type." u9 9 u16)
    ("The 10-bit unsigned integer type." u10 10 u16)
    ("The 11-bit unsigned integer type." u11 11 u16)
    ("The 12-bit unsigned integer type." u12 12 u16)
    ("The 13-bit unsigned integer type." u13 13 u16)
    ("The 14-bit unsigned integer type." u14 14 u16)
    ("The 15-bit unsigned integer type." u15 15 u16)
    ("The 17-bit unsigned integer type." u17 17 u32)
    ("The 18-bit unsigned integer type." u18 18 u32)
    ("The 19-bit unsigned integer type." u19 19 u32)
    ("The 20-bit unsigned integer type." u20 20 u32)
    ("The 21-bit unsigned integer type." u21 21 u32)
    ("The 22-bit unsigned integer type." u22 22 u32)
    ("The 23-bit unsigned integer type." u23 23 u32)
    ("The 24-bit unsigned integer type." u24 24 u32)
    ("The 25-bit unsigned integer type." u25 25 u32)
    ("The 26-bit unsigned integer type." u26 26 u32)
    ("The 27-bit unsigned integer type." u27 27 u32)
    ("The 28-bit unsigned integer type." u28 28 u32)
    ("The 29-bit unsigned integer type." u29 29 u32)
    ("The 30-bit unsigned integer type." u30 30 u32)
    ("The 31-bit unsigned integer type." u31 31 u32)
    ("The 33-bit unsigned integer type." u33 33 u64)
    ("The 34-bit unsigned integer type." u34 34 u64)
    ("The 35-bit unsigned integer type." u35 35 u64)
    ("The 36-bit unsigned integer type." u36 36 u64)
    ("The 37-bit unsigned integer type." u37 37 u64)
    ("The 38-bit unsigned integer type." u38 38 u64)
    ("The 39-bit unsigned integer type." u39 39 u64)
    ("The 40-bit unsigned integer type." u40 40 u64)
    ("The 41-bit unsigned integer type." u41 41 u64)
    ("The 42-bit unsigned integer type." u42 42 u64)
    ("The 43-bit unsigned integer type." u43 43 u64)
    ("The 44-bit unsigned integer type." u44 44 u64)
    ("The 45-bit unsigned integer type." u45 45 u64)
    ("The 46-bit unsigned integer type." u46 46 u64)
    ("The 47-bit unsigned integer type." u47 47 u64)
    ("The 48-bit unsigned integer type." u48 48 u64)
    ("The 49-bit unsigned integer type." u49 49 u64)
    ("The 50-bit unsigned integer type." u50 50 u64)
    ("The 51-bit unsigned integer type." u51 51 u64)
    ("The 52-bit unsigned integer type." u52 52 u64)
    ("The 53-bit unsigned integer type." u53 53 u64)
    ("The 54-bit unsigned integer type." u54 54 u64)
    ("The 55-bit unsigned integer type." u55 55 u64)
    ("The 56-bit unsigned integer type." u56 56 u64)
    ("The 57-bit unsigned integer type." u57 57 u64)
    ("The 58-bit unsigned integer type." u58 58 u64)
    ("The 59-bit unsigned integer type." u59 59 u64)
    ("The 60-bit unsigned integer type." u60 60 u64)
    ("The 61-bit unsigned integer type." u61 61 u64)
    ("The 62-bit unsigned integer type." u62 62 u64)
    ("The 63-bit unsigned integer type." u63 63 u64)
    ("The 65-bit unsigned integer type." u65 65 u128)
    ("The 66-bit unsigned integer type." u66 66 u128)
    ("The 67-bit unsigned integer type." u67 67 u128)
    ("The 68-bit unsigned integer type." u68 68 u128)
    ("The 69-bit unsigned integer type." u69 69 u128)
    ("The 70-bit unsigned integer type." u70 70 u128)
    ("The 71-bit unsigned integer type." u71 71 u128)
    ("The 72-bit unsigned integer type." u72 72 u128)
    ("The 73-bit unsigned integer type." u73 73 u128)
    ("The 74-bit unsigned integer type." u74 74 u128)
    ("The 75-bit unsigned integer type." u75 75 u128)
    ("The 76-bit unsigned integer type." u76 76 u128)
    ("The 77-bit unsigned integer type." u77 77 u128)
    ("The 78-bit unsigned integer type." u78 78 u128)
    ("The 79-bit unsigned integer type." u79 79 u128)
    ("The 80-bit unsigned integer type." u80 80 u128)
    ("The 81-bit unsigned integer type." u81 81 u128)
    ("The 82-bit unsigned integer type." u82 82 u128)
    ("The 83-bit unsigned integer type." u83 83 u128)
    ("The 84-bit unsigned integer type." u84 84 u128)
    ("The 85-bit unsigned integer type." u85 85 u128)
    ("The 86-bit unsigned integer type." u86 86 u128)
    ("The 87-bit unsigned integer type." u87 87 u128)
    ("The 88-bit unsigned integer type." u88 88 u128)
    ("The 89-bit unsigned integer type." u89 89 u128)
    ("The 90-bit unsigned integer type." u90 90 u128)
    ("The 91-bit unsigned integer type." u91 91 u128)
    ("The 92-bit unsigned integer type." u92 92 u128)
    ("The 93-bit unsigned integer type." u93 93 u128)
    ("The 94-bit unsigned integer type." u94 94 u128)
    ("The 95-bit unsigned integer type." u95 95 u128)
    ("The 96-bit unsigned integer type." u96 96 u128)
    ("The 97-bit unsigned integer type." u97 97 u128)
    ("The 98-bit unsigned integer type." u98 98 u128)
    ("The 99-bit unsigned integer type." u99 99 u128)
    ("The 100-bit unsigned integer type." u100 100 u128)
    ("The 101-bit unsigned integer type." u101 101 u128)
    ("The 102-bit unsigned integer type." u102 102 u128)
    ("The 103-bit unsigned integer type." u103 103 u128)
    ("The 104-bit unsigned integer type." u104 104 u128)
    ("The 105-bit unsigned integer type." u105 105 u128)
    ("The 106-bit unsigned integer type." u106 106 u128)
    ("The 107-bit unsigned integer type." u107 107 u128)
    ("The 108-bit unsigned integer type." u108 108 u128)
    ("The 109-bit unsigned integer type." u109 109 u128)
    ("The 110-bit unsigned integer type." u110 110 u128)
    ("The 111-bit unsigned integer type." u111 111 u128)
    ("The 112-bit unsigned integer type." u112 112 u128)
    ("The 113-bit unsigned integer type." u113 113 u128)
    ("The 114-bit unsigned integer type." u114 114 u128)
    ("The 115-bit unsigned integer type." u115 115 u128)
    ("The 116-bit unsigned integer type." u116 116 u128)
    ("The 117-bit unsigned integer type." u117 117 u128)
    ("The 118-bit unsigned integer type." u118 118 u128)
    ("The 119-bit unsigned integer type." u119 119 u128)
    ("The 120-bit unsigned integer type." u120 120 u128)
    ("The 121-bit unsigned integer type." u121 121 u128)
    ("The 122-bit unsigned integer type." u122 122 u128)
    ("The 123-bit unsigned integer type." u123 123 u128)
    ("The 124-bit unsigned integer type." u124 124 u128)
    ("The 125-bit unsigned integer type." u125 125 u128)
    ("The 126-bit unsigned integer type." u126 126 u128)
    ("The 127-bit unsigned integer type." u127 127 u128)
}

define_signed! {
    ("The 1-bit signed integer type." i1 1 i8)
    ("The 2-bit signed integer type." i2 2 i8)
    ("The 3-bit signed integer type." i3 3 i8)
    ("The 4-bit signed integer type." i4 4 i8)
    ("The 5-bit signed integer type." i5 5 i8)
    ("The 6-bit signed integer type." i6 6 i8)
    ("The 7-bit signed integer type." i7 7 i8)
    ("The 9-bit signed integer type." i9 9 i16)
    ("The 10-bit signed integer type." i10 10 i16)
    ("The 11-bit signed integer type." i11 11 i16)
    ("The 12-bit signed integer type." i12 12 i16)
    ("The 13-bit signed integer type." i13 13 i16)
    ("The 14-bit signed integer type." i14 14 i16)
    ("The 15-bit signed integer type." i15 15 i16)
    ("The 17-bit signed integer type." i17 17 i32)
    ("The 18-bit signed integer type." i18 18 i32)
    ("The 19-bit signed integer type." i19 19 i32)
    ("The 20-bit signed integer type." i20 20 i32)
    ("The 21-bit signed integer type." i21 21 i32)
    ("The 22-bit signed integer type." i22 22 i32)
    ("The 23-bit signed integer type." i23 23 i32)
    ("The 24-bit signed integer type." i24 24 i32)
    ("The 25-bit signed integer type." i25 25 i32)
    ("The 26-bit signed integer type." i26 26 i32)
    ("The 27-bit signed integer type." i27 27 i32)
    ("The 28-bit signed integer type." i28 28 i32)
    ("The 29-bit signed integer type." i29 29 i32)
    ("The 30-bit signed integer type." i30 30 i32)
    ("The 31-bit signed integer type." i31 31 i32)
    ("The 33-bit signed integer type." i33 33 i64)
    ("The 34-bit signed integer type." i34 34 i64)
    ("The 35-bit signed integer type." i35 35 i64)
    ("The 36-bit signed integer type." i36 36 i64)
    ("The 37-bit signed integer type." i37 37 i64)
    ("The 38-bit signed integer type." i38 38 i64)
    ("The 39-bit signed integer type." i39 39 i64)
    ("The 40-bit signed integer type." i40 40 i64)
    ("The 41-bit signed integer type." i41 41 i64)
    ("The 42-bit signed integer type." i42 42 i64)
    ("The 43-bit signed integer type." i43 43 i64)
    ("The 44-bit signed integer type." i44 44 i64)
    ("The 45-bit signed integer type." i45 45 i64)
    ("The 46-bit signed integer type." i46 46 i64)
    ("The 47-bit signed integer type." i47 47 i64)
    ("The 48-bit signed integer type." i48 48 i64)
    ("The 49-bit signed integer type." i49 49 i64)
    ("The 50-bit signed integer type." i50 50 i64)
    ("The 51-bit signed integer type." i51 51 i64)
    ("The 52-bit signed integer type." i52 52 i64)
    ("The 53-bit signed integer type." i53 53 i64)
    ("The 54-bit signed integer type." i54 54 i64)
    ("The 55-bit signed integer type." i55 55 i64)
    ("The 56-bit signed integer type." i56 56 i64)
    ("The 57-bit signed integer type." i57 57 i64)
    ("The 58-bit signed integer type." i58 58 i64)
    ("The 59-bit signed integer type." i59 59 i64)
    ("The 60-bit signed integer type." i60 60 i64)
    ("The 61-bit signed integer type." i61 61 i64)
    ("The 62-bit signed integer type." i62 62 i64)
    ("The 63-bit signed integer type." i63 63 i64)
    ("The 65-bit signed integer type." i65 65 i128)
    ("The 66-bit signed integer type." i66 66 i128)
    ("The 67-bit signed integer type." i67 67 i128)
    ("The 68-bit signed integer type." i68 68 i128)
    ("The 69-bit signed integer type." i69 69 i128)
    ("The 70-bit signed integer type." i70 70 i128)
    ("The 71-bit signed integer type." i71 71 i128)
    ("The 72-bit signed integer type." i72 72 i128)
    ("The 73-bit signed integer type." i73 73 i128)
    ("The 74-bit signed integer type." i74 74 i128)
    ("The 75-bit signed integer type." i75 75 i128)
    ("The 76-bit signed integer type." i76 76 i128)
    ("The 77-bit signed integer type." i77 77 i128)
    ("The 78-bit signed integer type." i78 78 i128)
    ("The 79-bit signed integer type." i79 79 i128)
    ("The 80-bit signed integer type." i80 80 i128)
    ("The 81-bit signed integer type." i81 81 i128)
    ("The 82-bit signed integer type." i82 82 i128)
    ("The 83-bit signed integer type." i83 83 i128)
    ("The 84-bit signed integer type." i84 84 i128)
    ("The 85-bit signed integer type." i85 85 i128)
    ("The 86-bit signed integer type." i86 86 i128)
    ("The 87-bit signed integer type." i87 87 i128)
    ("The 88-bit signed integer type." i88 88 i128)
    ("The 89-bit signed integer type." i89 89 i128)
    ("The 90-bit signed integer type." i90 90 i128)
    ("The 91-bit signed integer type." i91 91 i128)
    ("The 92-bit signed integer type." i92 92 i128)
    ("The 93-bit signed integer type." i93 93 i128)
    ("The 94-bit signed integer type." i94 94 i128)
    ("The 95-bit signed integer type." i95 95 i128)
    ("The 96-bit signed integer type." i96 96 i128)
    ("The 97-bit signed integer type." i97 97 i128)
    ("The 98-bit signed integer type." i98 98 i128)
    ("The 99-bit signed integer type." i99 99 i128)
    ("The 100-bit signed integer type." i100 100 i128)
    ("The 101-bit signed integer type." i101 101 i128)
    ("The 102-bit signed integer type." i102 102 i128)
    ("The 103-bit signed integer type." i103 103 i128)
    ("The 104-bit signed integer type." i104 104 i128)
    ("The 105-bit signed integer type." i105 105 i128)
    ("The 106-bit signed integer type." i106 106 i128)
    ("The 107-bit signed integer type." i107 107 i128)
    ("The 108-bit signed integer type." i108 108 i128)
    ("The 109-bit signed integer type." i109 109 i128)
    ("The 110-bit signed integer type." i110 110 i128)
    ("The 111-bit signed integer type." i111 111 i128)
    ("The 112-bit signed integer type." i112 112 i128)
    ("The 113-bit signed integer type." i113 113 i128)
    ("The 114-bit signed integer type." i114 114 i128)
    ("The 115-bit signed integer type." i115 115 i128)
    ("The 116-bit signed integer type." i116 116 i128)
    ("The 117-bit signed integer type." i117 117 i128)
    ("The 118-bit signed integer type." i118 118 i128)
    ("The 119-bit signed integer type." i119 119 i128)
    ("The 120-bit signed integer type." i120 120 i128)
    ("The 121-bit signed integer type." i121 121 i128)
    ("The 122-bit signed integer type." i122 122 i128)
    ("The 123-bit signed integer type." i123 123 i128)
    ("The 124-bit signed integer type." i124 124 i128)
    ("The 125-bit signed integer type." i125 125 i128)
    ("The 126-bit signed integer type." i126 126 i128)
    ("The 127-bit signed integer type." i127 127 i128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_values() {
        let a = u1::MAX;
        let b = u1::new(1);
        assert_eq!(a, b);
        assert_eq!(u1::MAX, u1::new(1));
        assert_eq!(u2::MAX, u2::new(3));
        assert_eq!(u3::MAX, u3::new(7));
        assert_eq!(u7::MAX, u7::new(127));
        assert_eq!(u9::MAX, u9::new(511));

        assert_eq!(i1::MAX, i1::new(0));
        assert_eq!(i2::MAX, i2::new(1));
        assert_eq!(i3::MAX, i3::new(3));
        assert_eq!(i7::MAX, i7::new(63));
        assert_eq!(i9::MAX, i9::new(255));

        assert_eq!(u1::MIN, u1::new(0));
        assert_eq!(u2::MIN, u2::new(0));
        assert_eq!(u3::MIN, u3::new(0));
        assert_eq!(u7::MIN, u7::new(0));
        assert_eq!(u9::MIN, u9::new(0));
        assert_eq!(u127::MIN, u127::new(0));

        assert_eq!(i1::MIN, i1::new(-1));
        assert_eq!(i2::MIN, i2::new(-2));
        assert_eq!(i3::MIN, i3::new(-4));
        assert_eq!(i7::MIN, i7::new(-64));
        assert_eq!(i9::MIN, i9::new(-256));
    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(u1::MAX.wrapping_add(u1::new(1)), u1::new(0));
        assert_eq!(u1::MAX.wrapping_add(u1::new(0)), u1::new(1));

        assert_eq!(u5::MAX.wrapping_add(u5::new(1)), u5::new(0));
        assert_eq!(u5::MAX.wrapping_add(u5::new(4)), u5::new(3));

        assert_eq!(u127::MAX.wrapping_add(u127::new(100)), u127::new(99));
        assert_eq!(u127::MAX.wrapping_add(u127::new(1)), u127::new(0));

        assert_eq!(i1::MAX.wrapping_add(i1::new(0)), i1::new(0));
        assert_eq!(i1::MAX.wrapping_add(i1::new(-1)), i1::new(-1));

        assert_eq!(i7::MAX.wrapping_add(i7::new(1)), i7::MIN);
        assert_eq!(i7::MAX.wrapping_add(i7::new(4)), i7::new(-61));
    }

    #[test]
    #[should_panic]
    fn test_add_overflow_u5() {
        let _s = u5::MAX + u5::new(1);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow_u127() {
        let _s = u127::MAX + u127::new(1);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow_i96() {
        let _s = i96::MAX + i96::new(100);
    }

    #[test]
    #[should_panic]
    fn test_add_underflow_i96() {
        let _s = i96::MIN + i96::new(-100);
    }

    #[test]
    #[should_panic]
    fn test_add_underflow_i17() {
        let _s = i17::MIN + i17::new(-1);
    }

    #[test]
    fn test_add() {
        assert_eq!(u5::new(1) + u5::new(2), u5::new(3));

        assert_eq!(i7::MAX + i7::MIN, i7::new(-1));
        assert_eq!(i7::new(4) + i7::new(-3), i7::new(1));
        assert_eq!(i7::new(-4) + i7::new(3), i7::new(-1));
        assert_eq!(i7::new(-3) + i7::new(-20), i7::new(-23));
    }

    #[test]
    #[should_panic]
    fn test_sub_overflow_i23() {
        let _s = i23::MIN - i23::MAX;
    }

    #[test]
    #[should_panic]
    fn test_sub_underflow_u5() {
        let _s = u5::MIN - u5::new(1);
    }

    #[test]
    #[should_panic]
    fn test_sub_underflow_i5() {
        let _s = i5::MIN - i5::new(1);
    }

    #[test]
    fn test_sub() {
        assert_eq!(u5::new(1) - u5::new(1), u5::new(0));
        assert_eq!(u5::new(3) - u5::new(2), u5::new(1));

        assert_eq!(i1::new(-1) - i1::new(-1), i1::new(0));
        assert_eq!(i7::MIN - i7::MIN, i7::new(0));
        assert_eq!(i7::new(4) - i7::new(-3), i7::new(7));
        assert_eq!(i7::new(-4) - i7::new(3), i7::new(-7));
        assert_eq!(i7::new(-3) - i7::new(-20), i7::new(17));
    }

    #[test]
    fn test_shr() {
        assert_eq!(u5::new(8) >> 1usize, u5::new(4));
        assert_eq!(u5::new(8) >> 1u8, u5::new(4));
        assert_eq!(u5::new(8) >> 1u16, u5::new(4));
        assert_eq!(u5::new(8) >> 1u32, u5::new(4));
        assert_eq!(u5::new(8) >> 1u64, u5::new(4));
        assert_eq!(u5::new(8) >> 1isize, u5::new(4));
        assert_eq!(u5::new(8) >> 1i8, u5::new(4));
        assert_eq!(u5::new(8) >> 1i16, u5::new(4));
        assert_eq!(u5::new(8) >> 1i32, u5::new(4));
        assert_eq!(u5::new(8) >> 1i64, u5::new(4));

        assert_eq!(u5::MAX >> 4, u5::new(1));

        assert_eq!(i7::new(-1) >> 5, i7::new(-1));
    }

    #[test]
    fn test_shl() {
        assert_eq!(u5::new(15) << 1usize, u5::new(30));
        assert_eq!(u5::new(15) << 1u8, u5::new(30));
        assert_eq!(u5::new(15) << 1u16, u5::new(30));
        assert_eq!(u5::new(15) << 1u32, u5::new(30));
        assert_eq!(u5::new(15) << 1u64, u5::new(30));
        assert_eq!(u5::new(15) << 1isize, u5::new(30));
        assert_eq!(u5::new(15) << 1i8, u5::new(30));
        assert_eq!(u5::new(15) << 1i16, u5::new(30));
        assert_eq!(u5::new(15) << 1i32, u5::new(30));
        assert_eq!(u5::new(15) << 1i64, u5::new(30));

        assert_eq!(u5::MAX << 4, u5::new(16));

        assert_eq!(i5::new(8) << 1, i5::new(-16));
        assert_eq!(i7::new(1) << 3, i7::new(8));
    }

    #[test]
    fn test_shr_assign() {
        let mut x = u10::new(512);
        x >>= 1usize;
        assert_eq!(x, u10::new(256));
        x >>= 1isize;
        assert_eq!(x, u10::new(128));
        x >>= 1u8;
        assert_eq!(x, u10::new(64));
        x >>= 1i8;
        assert_eq!(x, u10::new(32));
        x >>= 2u64;
        assert_eq!(x, u10::new(8));
        x >>= 3i32;
        assert_eq!(x, u10::new(1));
    }

    #[test]
    fn test_shl_assign() {
        let mut x = u9::new(1);
        x <<= 3i32;
        assert_eq!(x, u9::new(8));
        x <<= 2u64;
        assert_eq!(x, u9::new(32));
        x <<= 1usize;
        assert_eq!(x, u9::new(64));
        x <<= 1isize;
        assert_eq!(x, u9::new(128));
        x <<= 1u8;
        assert_eq!(x, u9::new(256));
    }

    #[test]
    fn test_bitor() {
        assert_eq!(u9::new(1) | u9::new(8), u9::new(9));
        assert_eq!(&u9::new(1) | u9::new(8), u9::new(9));
        assert_eq!(u9::new(1) | &u9::new(8), u9::new(9));
        assert_eq!(&u9::new(1) | &u9::new(8), u9::new(9));
    }

    #[test]
    fn test_bitor_assign() {
        let mut x = u12::new(4);
        x |= u12::new(1);
        assert_eq!(x, u12::new(5));
        x |= u12::new(128);
        assert_eq!(x, u12::new(133));
        x = u12::new(1);
        x |= u12::new(127);
        assert_eq!(x, u12::new(127));
    }

    #[test]
    fn test_bitxor() {
        assert_eq!(u7::new(0x7F) ^ u7::new(42), u7::new(85));
        assert_eq!(&u7::new(0) ^ u7::new(42), u7::new(42));
        assert_eq!(u7::new(0x10) ^ &u7::new(0x1), u7::new(0x11));
        assert_eq!(&u7::new(11) ^ &u7::new(1), u7::new(10));
    }

    #[test]
    fn test_bitxor_assign() {
        let mut x = u12::new(4);
        x ^= u12::new(1);
        assert_eq!(x, u12::new(5));
        x ^= u12::new(128);
        assert_eq!(x, u12::new(133));
        x ^= u12::new(1);
        assert_eq!(x, u12::new(132));
        x ^= u12::new(127);
        assert_eq!(x, u12::new(251));
    }

    #[test]
    fn test_bitand() {
        assert_eq!(i9::new(-7) & i9::new(-9), i9::from(-7i8 & -9i8));
        assert_eq!(&i9::new(-7) & i9::new(-9), i9::from(&-7i8 & -9i8));
        assert_eq!(i9::new(-7) & &i9::new(-9), i9::from(-7i8 & &-9i8));
        assert_eq!(&i9::new(-7) & &i9::new(-9), i9::from(&-7i8 & &-9i8));

        assert_eq!(u9::new(8) & u9::new(9), u9::new(8));
        assert_eq!(&u9::new(8) & u9::new(9), u9::new(8));
        assert_eq!(u9::new(8) & &u9::new(9), u9::new(8));
        assert_eq!(&u9::new(8) & &u9::new(9), u9::new(8));
    }

    #[test]
    fn test_bitand_assign() {
        let mut x = u12::new(255);
        x &= u12::new(127);
        assert_eq!(x, u12::new(127));
        x &= u12::new(7);
        assert_eq!(x, u12::new(7));
        x &= u12::new(127);
        assert_eq!(x, u12::new(7));
        x &= u12::new(4);
        assert_eq!(x, u12::new(4));
    }

    #[test]
    fn test_not() {
        assert_eq!(!u7::new(42), u7::new(85));
        assert_eq!(!u7::new(0x7F), u7::new(0));
        assert_eq!(!u7::new(0), u7::new(0x7F));
        assert_eq!(!u7::new(56), u7::new(71));
    }
}
