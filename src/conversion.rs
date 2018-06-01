use *;

// Only implement if $from can be converted into $name lossless
macro_rules! implement_from {
    {[$($name:ident),*], [$($from:ident),*] } => {$(implement_from!($name, $from);)*};
    {$name:ident, [$($from:ident),*] } => {$(implement_from!($name, $from);)*};
    {[$($name:ident),*], $from:ident } => {$(implement_from!($name, $from);)*};
    {$name:ident, $from:ty} => {
        impl From<$from> for $name {
            fn from(x: $from) -> $name {
                $name(x.into())
            }
        }
    };
}

// Only implement if $type can be converted from $name lossless
macro_rules! implement_into {
    {[$($name:ident),*], $from:ident } => {$(implement_into!($name, $from);)*};
    {$name:ident, $into:ident} => {
        impl From<$name> for $into {
            fn from(x: $name) -> $into {
                $into::from(x.0)
            }
        }
    };
}


// Implement From for all unsigned integers

implement_into!([u2, u3, u4, u5, u6, u7], u8);        
implement_from!([u9, u10, u11, u12, u13, u14, u15], u8);
implement_from!([u17, u18, u19, u20, u21, u22, u23, u24], u8);
implement_from!([u25, u26, u27, u28, u29, u30, u31], u8);
implement_from!([u33, u34, u35, u36, u37, u38, u39, u40], u8);
implement_from!([u41, u42, u43, u44, u45, u46, u47, u48], u8);
implement_from!([u49, u50, u51, u52, u53, u54, u55, u56], u8);
implement_from!([u57, u58, u59, u60, u61, u62, u63], u8);

implement_into!([u2, u3, u4, u5, u6, u7], u16);        
implement_into!([u9, u10, u11, u12, u13, u14, u15], u16);
implement_from!([u17, u18, u19, u20, u21, u22, u23, u24], u16);
implement_from!([u25, u26, u27, u28, u29, u30, u31], u16);
implement_from!([u33, u34, u35, u36, u37, u38, u39, u40], u16);
implement_from!([u41, u42, u43, u44, u45, u46, u47, u48], u16);
implement_from!([u49, u50, u51, u52, u53, u54, u55, u56], u16);
implement_from!([u57, u58, u59, u60, u61, u62, u63], u16);

implement_into!([u2, u3, u4, u5, u6, u7], u32);        
implement_into!([u9, u10, u11, u12, u13, u14, u15], u32);
implement_into!([u17, u18, u19, u20, u21, u22, u23, u24], u32);
implement_into!([u25, u26, u27, u28, u29, u30, u31], u32);
implement_from!([u33, u34, u35, u36, u37, u38, u39, u40], u32);
implement_from!([u41, u42, u43, u44, u45, u46, u47, u48], u32);
implement_from!([u49, u50, u51, u52, u53, u54, u55, u56], u32);
implement_from!([u57, u58, u59, u60, u61, u62, u63], u32);

implement_into!([u2, u3, u4, u5, u6, u7], u64);        
implement_into!([u9, u10, u11, u12, u13, u14, u15], u64);
implement_into!([u17, u18, u19, u20, u21, u22, u23, u24], u64);
implement_into!([u25, u26, u27, u28, u29, u30, u31], u64);
implement_into!([u33, u34, u35, u36, u37, u38, u39, u40], u64);
implement_into!([u41, u42, u43, u44, u45, u46, u47, u48], u64);
implement_into!([u49, u50, u51, u52, u53, u54, u55, u56], u64);
implement_into!([u57, u58, u59, u60, u61, u62, u63], u64);



implement_from!(u3, [u2]);
implement_from!(u4, [u2, u3]);
implement_from!(u5, [u2, u3, u4]);
implement_from!(u6, [u2, u3, u4, u5]);
implement_from!(u7, [u2, u3, u4, u5, u6]);

implement_from!(u9, [u2, u3, u4, u5, u6, u7]);
implement_from!(u10, [u2, u3, u4, u5, u6, u7, u9]);
implement_from!(u11, [u2, u3, u4, u5, u6, u7, u9, u10]);
implement_from!(u12, [u2, u3, u4, u5, u6, u7, u9, u10, u11]);
implement_from!(u13, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12]);
implement_from!(u14, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13]);
implement_from!(u15, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14]);

implement_from!(u17, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15]);
implement_from!(u18, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17]);
implement_from!(u19, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18]);
implement_from!(u20, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19]);
implement_from!(u21, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20]);
implement_from!(u22, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21]);
implement_from!(u23, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22]);
implement_from!(u24, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23]);

implement_from!(u25, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24]);
implement_from!(u26, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25]);
implement_from!(u27, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26]);
implement_from!(u28, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27]);
implement_from!(u29, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28]);
implement_from!(u30, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29]);
implement_from!(u31, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30]);

implement_from!(u33, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31]);
implement_from!(u34, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33]);
implement_from!(u35, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34]);
implement_from!(u36, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35]);
implement_from!(u37, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36]);
implement_from!(u38, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37]);
implement_from!(u39, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38]);
implement_from!(u40, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39]);
implement_from!(u41, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40]);

implement_from!(u42, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41]);
implement_from!(u43, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42]);
implement_from!(u44, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43]);
implement_from!(u45, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44]);
implement_from!(u46, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45]);
implement_from!(u47, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46]);
implement_from!(u48, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47]);
implement_from!(u49, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48]);
implement_from!(u50, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49]);
implement_from!(u51, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50]);
implement_from!(u52, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51]);
implement_from!(u53, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52]);
implement_from!(u54, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53]);
implement_from!(u55, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54]);
implement_from!(u56, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54, u55]);
implement_from!(u57, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54, u55, u56]);

implement_from!(u58, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54, u55, u56, u57]);
implement_from!(u59, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54, u55, u56, u57, u58]);
implement_from!(u60, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54, u55, u56, u57, u58, u59]);
implement_from!(u61, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54, u55, u56, u57, u58, u59, u60]);
implement_from!(u62, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                      u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54, u55, u56, u57, u58, u59, u60, u61]);
implement_from!(u63, [u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19, u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31,
                     u33, u34, u35, u36, u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52, u53, u54, u55, u56, u57, u58, u59, u60, u61, u62]);



// Implement From for all signed integer

implement_into!([i2, i3, i4, i5, i6, i7], i8);        
implement_from!([i9, i10, i11, i12, i13, i14, i15], i8);
implement_from!([i17, i18, i19, i20, i21, i22, i23, i24], i8);
implement_from!([i25, i26, i27, i28, i29, i30, i31], i8);
implement_from!([i33, i34, i35, i36, i37, i38, i39, i40], i8);
implement_from!([i41, i42, i43, i44, i45, i46, i47, i48], i8);
implement_from!([i49, i50, i51, i52, i53, i54, i55, i56], i8);
implement_from!([i57, i58, i59, i60, i61, i62, i63], i8);

implement_into!([i2, i3, i4, i5, i6, i7], i16);        
implement_into!([i9, i10, i11, i12, i13, i14, i15], i16);
implement_from!([i17, i18, i19, i20, i21, i22, i23, i24], i16);
implement_from!([i25, i26, i27, i28, i29, i30, i31], i16);
implement_from!([i33, i34, i35, i36, i37, i38, i39, i40], i16);
implement_from!([i41, i42, i43, i44, i45, i46, i47, i48], i16);
implement_from!([i49, i50, i51, i52, i53, i54, i55, i56], i16);
implement_from!([i57, i58, i59, i60, i61, i62, i63], i16);

implement_into!([i2, i3, i4, i5, i6, i7], i32);        
implement_into!([i9, i10, i11, i12, i13, i14, i15], i32);
implement_into!([i17, i18, i19, i20, i21, i22, i23, i24], i32);
implement_into!([i25, i26, i27, i28, i29, i30, i31], i32);
implement_from!([i33, i34, i35, i36, i37, i38, i39, i40], i32);
implement_from!([i41, i42, i43, i44, i45, i46, i47, i48], i32);
implement_from!([i49, i50, i51, i52, i53, i54, i55, i56], i32);
implement_from!([i57, i58, i59, i60, i61, i62, i63], i32);

implement_into!([i2, i3, i4, i5, i6, i7], i64);        
implement_into!([i9, i10, i11, i12, i13, i14, i15], i64);
implement_into!([i17, i18, i19, i20, i21, i22, i23, i24], i64);
implement_into!([i25, i26, i27, i28, i29, i30, i31], i64);
implement_into!([i33, i34, i35, i36, i37, i38, i39, i40], i64);
implement_into!([i41, i42, i43, i44, i45, i46, i47, i48], i64);
implement_into!([i49, i50, i51, i52, i53, i54, i55, i56], i64);
implement_into!([i57, i58, i59, i60, i61, i62, i63], i64);



implement_from!(i3, [i2]);
implement_from!(i4, [i2, i3]);
implement_from!(i5, [i2, i3, i4]);
implement_from!(i6, [i2, i3, i4, i5]);
implement_from!(i7, [i2, i3, i4, i5, i6]);

implement_from!(i9, [i2, i3, i4, i5, i6, i7]);
implement_from!(i10, [i2, i3, i4, i5, i6, i7, i9]);
implement_from!(i11, [i2, i3, i4, i5, i6, i7, i9, i10]);
implement_from!(i12, [i2, i3, i4, i5, i6, i7, i9, i10, i11]);
implement_from!(i13, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12]);
implement_from!(i14, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13]);
implement_from!(i15, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14]);

implement_from!(i17, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15]);
implement_from!(i18, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17]);
implement_from!(i19, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18]);
implement_from!(i20, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19]);
implement_from!(i21, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20]);
implement_from!(i22, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21]);
implement_from!(i23, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22]);
implement_from!(i24, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23]);

implement_from!(i25, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24]);
implement_from!(i26, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25]);
implement_from!(i27, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26]);
implement_from!(i28, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27]);
implement_from!(i29, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28]);
implement_from!(i30, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29]);
implement_from!(i31, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30]);

implement_from!(i33, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31]);
implement_from!(i34, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33]);
implement_from!(i35, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34]);
implement_from!(i36, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35]);
implement_from!(i37, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36]);
implement_from!(i38, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37]);
implement_from!(i39, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38]);
implement_from!(i40, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39]);
implement_from!(i41, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40]);

implement_from!(i42, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41]);
implement_from!(i43, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42]);
implement_from!(i44, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43]);
implement_from!(i45, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44]);
implement_from!(i46, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45]);
implement_from!(i47, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46]);
implement_from!(i48, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47]);
implement_from!(i49, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48]);
implement_from!(i50, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49]);
implement_from!(i51, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50]);
implement_from!(i52, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51]);
implement_from!(i53, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52]);
implement_from!(i54, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53]);
implement_from!(i55, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54]);
implement_from!(i56, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54, i55]);
implement_from!(i57, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54, i55, i56]);

implement_from!(i58, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54, i55, i56, i57]);
implement_from!(i59, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54, i55, i56, i57, i58]);
implement_from!(i60, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54, i55, i56, i57, i58, i59]);
implement_from!(i61, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54, i55, i56, i57, i58, i59, i60]);
implement_from!(i62, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                      i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54, i55, i56, i57, i58, i59, i60, i61]);
implement_from!(i63, [i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31,
                     i33, i34, i35, i36, i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52, i53, i54, i55, i56, i57, i58, i59, i60, i61, i62]);

impl From<bool> for u1 {
    fn from(b: bool) -> Self {
        match b {
            true => u1(1),
            false => u1(0),
        }
    }
}

impl From<u1> for bool {
    fn from(u1(x): u1) -> Self {
        match x {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conversion_unsigned() {
        assert_eq!(u16::from(u9(12)), 12u16);
        assert_eq!(u32::from(u9(12)), 12u32);
        
        assert_eq!(u9(127), 127u8.into());
        
        assert_eq!(u7::from(u6(65)), u7(65));

    }
    
    #[test]
    fn test_conversion_signed() {
        assert_eq!(i16::from(i9(12)), 12i16);
        assert_eq!(i32::from(i9(12)), 12i32);
        
        assert_eq!(i16::from(i9(-12)), -12i16);
        assert_eq!(i32::from(i9(-12)), -12i32);
        
        assert_eq!(i9(127), 127i8.into());
        
        assert_eq!(i7::from(i6(65)), i7(65));
        assert_eq!(i7::from(i6(-65)), i7(-65));

    }
    
}
