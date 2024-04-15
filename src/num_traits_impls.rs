use num_traits::{ConstOne, ConstZero, One, Zero};

use crate::*;

macro_rules! impl_num_traits {
    ($type:ty$(, $rest:tt)*) => {
        impl One for $type {
            fn one() -> Self {
                <$type>::new(1)
            }
        }

        impl ConstOne for $type {
            const ONE: Self = <$type>::new(1);
        }

        impl Zero for $type {
            fn zero() -> Self {
                <$type>::new(0)
            }

            fn is_zero(&self) -> bool {
                self.0 == 0
            }
        }

        impl ConstZero for $type {
            const ZERO: Self = <$type>::new(0);
        }
        impl_num_traits!($($rest),*);
    };
    () => {}
}

impl_num_traits!(u1, u2, u3, u4, u5, u6, u7, u9, u10, u11, u12, u13, u14, u15, u17, u18, u19);
impl_num_traits!(u20, u21, u22, u23, u24, u25, u26, u27, u28, u29, u30, u31, u33, u34, u35, u36);
impl_num_traits!(u37, u38, u39, u40, u41, u42, u43, u44, u45, u46, u47, u48, u49, u50, u51, u52);
impl_num_traits!(u53, u54, u55, u56, u57, u58, u59, u60, u61, u62, u63, u65, u66, u67, u68, u69);
impl_num_traits!(u70, u71, u72, u73, u74, u75, u76, u77, u78, u79, u80, u81, u82, u83, u84, u85);
impl_num_traits!(u86, u87, u88, u89, u90, u91, u92, u93, u94, u95, u96, u97, u98, u99, u100, u101);
impl_num_traits!(u102, u103, u104, u105, u106, u107, u108, u109, u110, u111, u112, u113, u114);
impl_num_traits!(u115, u116, u117, u118, u119, u120, u121, u122, u123, u124, u125, u126, u127);

impl_num_traits!(i1, i2, i3, i4, i5, i6, i7, i9, i10, i11, i12, i13, i14, i15, i17, i18, i19);
impl_num_traits!(i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31, i33, i34, i35, i36);
impl_num_traits!(i37, i38, i39, i40, i41, i42, i43, i44, i45, i46, i47, i48, i49, i50, i51, i52);
impl_num_traits!(i53, i54, i55, i56, i57, i58, i59, i60, i61, i62, i63, i65, i66, i67, i68, i69);
impl_num_traits!(i70, i71, i72, i73, i74, i75, i76, i77, i78, i79, i80, i81, i82, i83, i84, i85);
impl_num_traits!(i86, i87, i88, i89, i90, i91, i92, i93, i94, i95, i96, i97, i98, i99, i100, i101);
impl_num_traits!(i102, i103, i104, i105, i106, i107, i108, i109, i110, i111, i112, i113, i114);
impl_num_traits!(i115, i116, i117, i118, i119, i120, i121, i122, i123, i124, i125, i126, i127);
