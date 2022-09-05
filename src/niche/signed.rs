use super::new_type;
use super::signed_bytes::*;
use super::unsigned_bytes::Max8 as MaxU8;

new_type! {
    (i1 i8)
    (-0x01 ..= 0x00)
    (a: Max1)
    (a: Max1)

    (i2 i8)
    (-0x02 ..= 0x01)
    (a: Max2)
    (a: Max2)

    (i3 i8)
    (-0x04 ..= 0x03)
    (a: Max3)
    (a: Max3)

    (i4 i8)
    (-0x08 ..= 0x07)
    (a: Max4)
    (a: Max4)

    (i5 i8)
    (-0x10 ..= 0x0f)
    (a: Max5)
    (a: Max5)

    (i6 i8)
    (-0x20 ..= 0x1f)
    (a: Max6)
    (a: Max6)

    (i7 i8)
    (-0x40 ..= 0x3f)
    (a: Max7)
    (a: Max7)

    // (i8 i8)
    // (-0x80 ..= 0x7f)
    // (a: Max8)
    // (a: Max8)

    (i9 i16)
    (-0x0100 ..= 0x00ff)
    (a: MaxU8, b: Max1)
    (b: Max1, a: MaxU8)

    (i10 i16)
    (-0x0200 ..= 0x01ff)
    (a: MaxU8, b: Max2)
    (b: Max2, a: MaxU8)

    (i11 i16)
    (-0x0400 ..= 0x03ff)
    (a: MaxU8, b: Max3)
    (b: Max3, a: MaxU8)

    (i12 i16)
    (-0x0800 ..= 0x07ff)
    (a: MaxU8, b: Max4)
    (b: Max4, a: MaxU8)

    (i13 i16)
    (-0x1000 ..= 0x0fff)
    (a: MaxU8, b: Max5)
    (b: Max5, a: MaxU8)

    (i14 i16)
    (-0x2000 ..= 0x1fff)
    (a: MaxU8, b: Max6)
    (b: Max6, a: MaxU8)

    (i15 i16)
    (-0x4000 ..= 0x3fff)
    (a: MaxU8, b: Max7)
    (b: Max7, a: MaxU8)

    // (i16 i16)
    // (-0x8000 ..= 0x7fff)
    // (a: MaxU8, b: Max8)
    // (b: Max8, a: MaxU8)

    (i17 i32)
    (-0x0001_0000 ..= 0x0000_ffff)
    (a: MaxU8, b: MaxU8, c: Max1, d: Max1)
    (d: Max1, c: Max1, b: MaxU8, a: MaxU8)

    (i18 i32)
    (-0x0002_0000 ..= 0x0001_ffff)
    (a: MaxU8, b: MaxU8, c: Max2, d: Max1)
    (d: Max1, c: Max2, b: MaxU8, a: MaxU8)

    (i19 i32)
    (-0x0004_0000 ..= 0x0003_ffff)
    (a: MaxU8, b: MaxU8, c: Max3, d: Max1)
    (d: Max1, c: Max3, b: MaxU8, a: MaxU8)

    (i20 i32)
    (-0x0008_0000 ..= 0x0007_ffff)
    (a: MaxU8, b: MaxU8, c: Max4, d: Max1)
    (d: Max1, c: Max4, b: MaxU8, a: MaxU8)

    (i21 i32)
    (-0x0010_0000 ..= 0x000f_ffff)
    (a: MaxU8, b: MaxU8, c: Max5, d: Max1)
    (d: Max1, c: Max5, b: MaxU8, a: MaxU8)

    (i22 i32)
    (-0x0020_0000 ..= 0x001f_ffff)
    (a: MaxU8, b: MaxU8, c: Max6, d: Max1)
    (d: Max1, c: Max6, b: MaxU8, a: MaxU8)

    (i23 i32)
    (-0x0040_0000 ..= 0x003f_ffff)
    (a: MaxU8, b: MaxU8, c: Max7, d: Max1)
    (d: Max1, c: Max7, b: MaxU8, a: MaxU8)

    (i24 i32)
    (-0x0080_0000 ..= 0x007f_ffff)
    (a: MaxU8, b: MaxU8, c: Max8, d: Max1)
    (d: Max1, c: MaxU8, b: MaxU8, a: MaxU8)

    (i25 i32)
    (-0x0100_0000 ..= 0x00ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: Max1)
    (d: Max1, c: MaxU8, b: MaxU8, a: MaxU8)

    (i26 i32)
    (-0x0200_0000 ..= 0x01ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: Max2)
    (d: Max2, c: MaxU8, b: MaxU8, a: MaxU8)

    (i27 i32)
    (-0x0400_0000 ..= 0x03ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: Max3)
    (d: Max3, c: MaxU8, b: MaxU8, a: MaxU8)

    (i28 i32)
    (-0x0800_0000 ..= 0x07ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: Max4)
    (d: Max4, c: MaxU8, b: MaxU8, a: MaxU8)

    (i29 i32)
    (-0x1000_0000 ..= 0x0fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: Max5)
    (d: Max5, c: MaxU8, b: MaxU8, a: MaxU8)

    (i30 i32)
    (-0x2000_0000 ..= 0x1fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: Max6)
    (d: Max6, c: MaxU8, b: MaxU8, a: MaxU8)

    (i31 i32)
    (-0x4000_0000 ..= 0x3fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: Max7)
    (d: Max7, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i32 i32)
    // (-0x8000_0000 ..= 0x7fff_ffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: Max8)
    // (d: Max8, c: MaxU8, b: MaxU8, a: MaxU8)
}

/*

e = f = g = h = 0
for i in range(33, 65):
    if e < 8:
        e += 1
    elif f < 8:
        f += 1
    elif g < 8:
        g += 1
    else:
        h += 1
    def M(i, j):
        if i == 0:
            return 'Max1'
        elif (i == 8) and (j > 0):
            return 'MaxU8'
        else:
            return f'Max{i}'
    print('new_type! {')
    print(f'    (I{i} Buf{i} i64)')
    print(f'    (-0x{2**(i - 1):016x} ..= 0x{2**(i - 1) - 1:016x})')
    print(f'    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: {M(e, f)}, f: {M(f, g)}, g: {M(g, h)}, h: Max{h or 1})')
    print(f'    (h: Max{h or 1}, g: {M(g, h)}, f: {M(f, e)}, e: {M(e, f)}, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)')
    print('}')

*/

new_type! {
    (i33 i64)
    (-0x0000000100000000 ..= 0x00000000ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: Max1, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: Max1, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i34 i64)
    (-0x0000000200000000 ..= 0x00000001ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: Max2, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: Max2, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i35 i64)
    (-0x0000000400000000 ..= 0x00000003ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: Max3, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: Max3, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i36 i64)
    (-0x0000000800000000 ..= 0x00000007ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: Max4, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: Max4, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i37 i64)
    (-0x0000001000000000 ..= 0x0000000fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: Max5, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: Max5, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i38 i64)
    (-0x0000002000000000 ..= 0x0000001fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: Max6, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: Max6, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i39 i64)
    (-0x0000004000000000 ..= 0x0000003fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: Max7, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: Max7, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i40 i64)
    (-0x0000008000000000 ..= 0x0000007fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: Max8, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: Max8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i41 i64)
    (-0x0000010000000000 ..= 0x000000ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: Max1, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max1, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i42 i64)
    (-0x0000020000000000 ..= 0x000001ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: Max2, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max2, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i43 i64)
    (-0x0000040000000000 ..= 0x000003ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: Max3, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max3, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i44 i64)
    (-0x0000080000000000 ..= 0x000007ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: Max4, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max4, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i45 i64)
    (-0x0000100000000000 ..= 0x00000fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: Max5, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max5, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i46 i64)
    (-0x0000200000000000 ..= 0x00001fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: Max6, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max6, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i47 i64)
    (-0x0000400000000000 ..= 0x00003fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: Max7, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: Max7, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i48 i64)
    (-0x0000800000000000 ..= 0x00007fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: Max8, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i49 i64)
    (-0x0001000000000000 ..= 0x0000ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: Max1, h: Max1)
    (h: Max1, g: Max1, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i50 i64)
    (-0x0002000000000000 ..= 0x0001ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: Max2, h: Max1)
    (h: Max1, g: Max2, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i51 i64)
    (-0x0004000000000000 ..= 0x0003ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: Max3, h: Max1)
    (h: Max1, g: Max3, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i52 i64)
    (-0x0008000000000000 ..= 0x0007ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: Max4, h: Max1)
    (h: Max1, g: Max4, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i53 i64)
    (-0x0010000000000000 ..= 0x000fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: Max5, h: Max1)
    (h: Max1, g: Max5, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i54 i64)
    (-0x0020000000000000 ..= 0x001fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: Max6, h: Max1)
    (h: Max1, g: Max6, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i55 i64)
    (-0x0040000000000000 ..= 0x003fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: Max7, h: Max1)
    (h: Max1, g: Max7, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i56 i64)
    (-0x0080000000000000 ..= 0x007fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: Max8, h: Max1)
    (h: Max1, g: Max8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i57 i64)
    (-0x0100000000000000 ..= 0x00ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: Max1)
    (h: Max1, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i58 i64)
    (-0x0200000000000000 ..= 0x01ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: Max2)
    (h: Max2, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i59 i64)
    (-0x0400000000000000 ..= 0x03ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: Max3)
    (h: Max3, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i60 i64)
    (-0x0800000000000000 ..= 0x07ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: Max4)
    (h: Max4, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i61 i64)
    (-0x1000000000000000 ..= 0x0fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: Max5)
    (h: Max5, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i62 i64)
    (-0x2000000000000000 ..= 0x1fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: Max6)
    (h: Max6, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i63 i64)
    (-0x4000000000000000 ..= 0x3fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: Max7)
    (h: Max7, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i64 i64)
    // (-0x8000000000000000 ..= 0x7fffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: Max8)
    // (h: Max8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

/*

i = j = k = l = m = n = o = p = 0
for I in range(65, 129):
    if i < 8:
        i += 1
    elif j < 8:
        j += 1
    elif k < 8:
        k += 1
    elif j < 8:
        j += 1
    elif l < 8:
        l += 1
    elif m < 8:
        m += 1
    elif n < 8:
        n += 1
    elif o < 8:
        o += 1
    else:
        p += 1
    def M(i, j):
        if i == 0:
            return 'Max1'
        elif (i == 8) and (j > 0):
            return 'MaxU8'
        else:
            return f'Max{i}'
    print(f'    (i{I} i128)')
    print(f'    (-0x{2**(I - 1):032x} ..= 0x{2**(I - 1) - 1:032x})')
    print(f'    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: {M(i, j)}, j: {M(j, k)}, k: {M(k, l)}, l: {M(l, m)}, m: {M(m, n)}, n: {M(n, o)}, o: {M(o, p)}, p: Max{p or 1})')
    print(f'    (p: Max{p or 1}, o: {M(o, p)}, n: {M(n, o)}, m: {M(m, n)}, l: {M(l, m)}, k: {M(k, l)}, j: {M(j, k)}, i: {M(i, j)}, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)')
    print()

*/

    (i65 i128)
    (-0x00000000000000010000000000000000 ..= 0x0000000000000000ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: Max1, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: Max1, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i66 i128)
    (-0x00000000000000020000000000000000 ..= 0x0000000000000001ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: Max2, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: Max2, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i67 i128)
    (-0x00000000000000040000000000000000 ..= 0x0000000000000003ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: Max3, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: Max3, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i68 i128)
    (-0x00000000000000080000000000000000 ..= 0x0000000000000007ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: Max4, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: Max4, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i69 i128)
    (-0x00000000000000100000000000000000 ..= 0x000000000000000fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: Max5, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: Max5, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i70 i128)
    (-0x00000000000000200000000000000000 ..= 0x000000000000001fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: Max6, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: Max6, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i71 i128)
    (-0x00000000000000400000000000000000 ..= 0x000000000000003fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: Max7, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: Max7, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i72 i128)
    (-0x00000000000000800000000000000000 ..= 0x000000000000007fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: Max8, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: Max8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i73 i128)
    (-0x00000000000001000000000000000000 ..= 0x00000000000000ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: Max1, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max1, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i74 i128)
    (-0x00000000000002000000000000000000 ..= 0x00000000000001ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: Max2, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max2, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i75 i128)
    (-0x00000000000004000000000000000000 ..= 0x00000000000003ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: Max3, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max3, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i76 i128)
    (-0x00000000000008000000000000000000 ..= 0x00000000000007ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: Max4, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max4, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i77 i128)
    (-0x00000000000010000000000000000000 ..= 0x0000000000000fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: Max5, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max5, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i78 i128)
    (-0x00000000000020000000000000000000 ..= 0x0000000000001fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: Max6, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max6, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i79 i128)
    (-0x00000000000040000000000000000000 ..= 0x0000000000003fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: Max7, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max7, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i80 i128)
    (-0x00000000000080000000000000000000 ..= 0x0000000000007fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: Max8, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: Max8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i81 i128)
    (-0x00000000000100000000000000000000 ..= 0x000000000000ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: Max1, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max1, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i82 i128)
    (-0x00000000000200000000000000000000 ..= 0x000000000001ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: Max2, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max2, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i83 i128)
    (-0x00000000000400000000000000000000 ..= 0x000000000003ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: Max3, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max3, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i84 i128)
    (-0x00000000000800000000000000000000 ..= 0x000000000007ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: Max4, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max4, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i85 i128)
    (-0x00000000001000000000000000000000 ..= 0x00000000000fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: Max5, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max5, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i86 i128)
    (-0x00000000002000000000000000000000 ..= 0x00000000001fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: Max6, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max6, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i87 i128)
    (-0x00000000004000000000000000000000 ..= 0x00000000003fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: Max7, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max7, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i88 i128)
    (-0x00000000008000000000000000000000 ..= 0x00000000007fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: Max8, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: Max8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i89 i128)
    (-0x00000000010000000000000000000000 ..= 0x0000000000ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: Max1, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max1, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i90 i128)
    (-0x00000000020000000000000000000000 ..= 0x0000000001ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: Max2, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max2, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i91 i128)
    (-0x00000000040000000000000000000000 ..= 0x0000000003ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: Max3, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max3, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i92 i128)
    (-0x00000000080000000000000000000000 ..= 0x0000000007ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: Max4, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max4, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i93 i128)
    (-0x00000000100000000000000000000000 ..= 0x000000000fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: Max5, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max5, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i94 i128)
    (-0x00000000200000000000000000000000 ..= 0x000000001fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: Max6, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max6, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i95 i128)
    (-0x00000000400000000000000000000000 ..= 0x000000003fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: Max7, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max7, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i96 i128)
    (-0x00000000800000000000000000000000 ..= 0x000000007fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: Max8, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: Max8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i97 i128)
    (-0x00000001000000000000000000000000 ..= 0x00000000ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: Max1, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max1, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i98 i128)
    (-0x00000002000000000000000000000000 ..= 0x00000001ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: Max2, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max2, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i99 i128)
    (-0x00000004000000000000000000000000 ..= 0x00000003ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: Max3, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max3, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i100 i128)
    (-0x00000008000000000000000000000000 ..= 0x00000007ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: Max4, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max4, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i101 i128)
    (-0x00000010000000000000000000000000 ..= 0x0000000fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: Max5, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max5, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i102 i128)
    (-0x00000020000000000000000000000000 ..= 0x0000001fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: Max6, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max6, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i103 i128)
    (-0x00000040000000000000000000000000 ..= 0x0000003fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: Max7, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max7, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i104 i128)
    (-0x00000080000000000000000000000000 ..= 0x0000007fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: Max8, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: Max8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i105 i128)
    (-0x00000100000000000000000000000000 ..= 0x000000ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: Max1, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max1, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i106 i128)
    (-0x00000200000000000000000000000000 ..= 0x000001ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: Max2, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max2, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i107 i128)
    (-0x00000400000000000000000000000000 ..= 0x000003ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: Max3, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max3, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i108 i128)
    (-0x00000800000000000000000000000000 ..= 0x000007ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: Max4, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max4, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i109 i128)
    (-0x00001000000000000000000000000000 ..= 0x00000fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: Max5, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max5, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i110 i128)
    (-0x00002000000000000000000000000000 ..= 0x00001fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: Max6, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max6, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i111 i128)
    (-0x00004000000000000000000000000000 ..= 0x00003fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: Max7, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max7, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i112 i128)
    (-0x00008000000000000000000000000000 ..= 0x00007fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: Max8, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i113 i128)
    (-0x00010000000000000000000000000000 ..= 0x0000ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i114 i128)
    (-0x00020000000000000000000000000000 ..= 0x0001ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: Max2, p: Max1)
    (p: Max1, o: Max2, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i115 i128)
    (-0x00040000000000000000000000000000 ..= 0x0003ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: Max3, p: Max1)
    (p: Max1, o: Max3, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i116 i128)
    (-0x00080000000000000000000000000000 ..= 0x0007ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: Max4, p: Max1)
    (p: Max1, o: Max4, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i117 i128)
    (-0x00100000000000000000000000000000 ..= 0x000fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: Max5, p: Max1)
    (p: Max1, o: Max5, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i118 i128)
    (-0x00200000000000000000000000000000 ..= 0x001fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: Max6, p: Max1)
    (p: Max1, o: Max6, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i119 i128)
    (-0x00400000000000000000000000000000 ..= 0x003fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: Max7, p: Max1)
    (p: Max1, o: Max7, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i120 i128)
    (-0x00800000000000000000000000000000 ..= 0x007fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: Max8, p: Max1)
    (p: Max1, o: Max8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i121 i128)
    (-0x01000000000000000000000000000000 ..= 0x00ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: Max1)
    (p: Max1, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i122 i128)
    (-0x02000000000000000000000000000000 ..= 0x01ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: Max2)
    (p: Max2, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i123 i128)
    (-0x04000000000000000000000000000000 ..= 0x03ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: Max3)
    (p: Max3, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i124 i128)
    (-0x08000000000000000000000000000000 ..= 0x07ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: Max4)
    (p: Max4, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i125 i128)
    (-0x10000000000000000000000000000000 ..= 0x0fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: Max5)
    (p: Max5, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i126 i128)
    (-0x20000000000000000000000000000000 ..= 0x1fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: Max6)
    (p: Max6, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i127 i128)
    (-0x40000000000000000000000000000000 ..= 0x3fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: Max7)
    (p: Max7, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i128 i128)
    // (-0x80000000000000000000000000000000 ..= 0x7fffffffffffffffffffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: Max8)
    // (p: Max8, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)
}
