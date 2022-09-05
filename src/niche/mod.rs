mod bytes;
mod signed;
mod unsigned;

pub use signed::*;
pub use unsigned::*;

macro_rules! new_type {
    (
        $(
            ( $Name:ident $Primitive:ident )
            ( $min:literal ..= $max:literal )
            ( $($LittleField:ident: $LittleType:ident),* )
            ( $($BigField:ident: $BigType:ident),* )
        )*
    ) => {$(
        #[repr(C)]
        #[cfg(target_endian = "little")]
        #[derive(Copy, Clone)]
        pub struct $Name {
            align: [core::primitive::$Primitive; 0],
            $($LittleField: $LittleType,)*
        }

        #[repr(C)]
        #[cfg(target_endian = "big")]
        #[derive(Copy, Clone)]
        pub struct $Name {
            align: [core::primitive::$Primitive; 0],
            $($BigField: $BigType,)*
        }

        impl $Name {
            pub const MIN: core::primitive::$Primitive = $min;
            pub const MAX: core::primitive::$Primitive = $max;
        }
    )*};
}

macro_rules! new_byte {
    ($Name:ident($Primitive:ident) { $($Variant:ident = $value:literal,)* }) => {
        #[allow(dead_code)]
        #[derive(Copy, Clone)]
        #[repr($Primitive)]
        pub(super) enum $Name {
            $($Variant = $value,)*
        }
    }
}

pub(crate) use {new_byte, new_type};
