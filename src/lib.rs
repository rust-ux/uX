#![cfg_attr(not(feature="std"), no_std)]


mod lib {
    pub mod core {
        #[cfg(feature="std")]
        pub use std::*;
        #[cfg(not(feature="std"))]
        pub use core::*;
    }
}

macro_rules! define_unsigned {
    ($name:ident, $bits:expr, $type:ty) => {
        #[allow(non_camel_case_types)]
        #[derive(Default, Clone, Copy, Debug)]
        pub struct $name($type);

        impl $name {
            pub const MAX: Self = $name(((1 as $type) << $bits) -1 );
            pub const MIN: Self = $name(0 as $type);

            fn mask(self) -> Self {
                $name(self.0 & ( ((1 as $type) << $bits) -1))
            }
        }
        
        implement_common_traits!($name, $bits, $type);
        
    }
}

macro_rules! implement_common_traits {
    ($name:ident, $bits:expr, $type:ty) => {
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.mask().0 == other.mask().0
            }
        }
        
        impl Eq for $name {}
        
    };
}


define_unsigned!(u2, 2, u8);
define_unsigned!(u3, 3, u8);
define_unsigned!(u4, 4, u8);
define_unsigned!(u5, 5, u8);
define_unsigned!(u6, 6, u8);
define_unsigned!(u7, 7, u8);

define_unsigned!(u9, 9, u16);
define_unsigned!(u10, 10, u16);
define_unsigned!(u11, 11, u16);
define_unsigned!(u12, 12, u16);
define_unsigned!(u13, 13, u16);
define_unsigned!(u14, 14, u16);
define_unsigned!(u15, 15, u16);

define_unsigned!(u17, 17, u32);
define_unsigned!(u18, 18, u32);
define_unsigned!(u19, 19, u32);
define_unsigned!(u20, 20, u32);
define_unsigned!(u21, 21, u32);
define_unsigned!(u22, 22, u32);
define_unsigned!(u23, 23, u32);
define_unsigned!(u24, 24, u32);

define_unsigned!(u25, 25, u32);
define_unsigned!(u26, 26, u32);
define_unsigned!(u27, 27, u32);
define_unsigned!(u28, 28, u32);
define_unsigned!(u29, 29, u32);
define_unsigned!(u30, 30, u32);
define_unsigned!(u31, 31, u32);

define_unsigned!(u33, 33, u64);
define_unsigned!(u34, 34, u64);
define_unsigned!(u35, 35, u64);
define_unsigned!(u36, 36, u64);
define_unsigned!(u37, 37, u64);
define_unsigned!(u38, 38, u64);
define_unsigned!(u39, 39, u64);
define_unsigned!(u40, 40, u64);

define_unsigned!(u41, 41, u64);
define_unsigned!(u42, 42, u64);
define_unsigned!(u43, 43, u64);
define_unsigned!(u44, 44, u64);
define_unsigned!(u45, 45, u64);
define_unsigned!(u46, 46, u64);
define_unsigned!(u47, 47, u64);
define_unsigned!(u48, 48, u64);

define_unsigned!(u49, 49, u64);
define_unsigned!(u50, 50, u64);
define_unsigned!(u51, 51, u64);
define_unsigned!(u52, 52, u64);
define_unsigned!(u53, 53, u64);
define_unsigned!(u54, 54, u64);
define_unsigned!(u55, 55, u64);
define_unsigned!(u56, 56, u64);

define_unsigned!(u57, 57, u64);
define_unsigned!(u58, 58, u64);
define_unsigned!(u59, 59, u64);
define_unsigned!(u60, 60, u64);
define_unsigned!(u61, 61, u64);
define_unsigned!(u62, 62, u64);
define_unsigned!(u63, 63, u64);
                
            
