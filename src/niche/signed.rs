use super::new_type;
use super::bytes::*;

new_type! {
    (i1 i8)
    (-0x01 ..= 0x00)
    (a: MaxI1)
    (a: MaxI1)

    (i2 i8)
    (-0x02 ..= 0x01)
    (a: MaxI2)
    (a: MaxI2)

    (i3 i8)
    (-0x04 ..= 0x03)
    (a: MaxI3)
    (a: MaxI3)

    (i4 i8)
    (-0x08 ..= 0x07)
    (a: MaxI4)
    (a: MaxI4)

    (i5 i8)
    (-0x10 ..= 0x0f)
    (a: MaxI5)
    (a: MaxI5)

    (i6 i8)
    (-0x20 ..= 0x1f)
    (a: MaxI6)
    (a: MaxI6)

    (i7 i8)
    (-0x40 ..= 0x3f)
    (a: MaxI7)
    (a: MaxI7)

    // (i8 i8)
    // (-0x80 ..= 0x7f)
    // (a: MaxI8)
    // (a: MaxI8)

    (i9 i16)
    (-0x0100 ..= 0x00ff)
    (a: MaxU8, b: MaxI1)
    (b: MaxI1, a: MaxU8)

    (i10 i16)
    (-0x0200 ..= 0x01ff)
    (a: MaxU8, b: MaxI2)
    (b: MaxI2, a: MaxU8)

    (i11 i16)
    (-0x0400 ..= 0x03ff)
    (a: MaxU8, b: MaxI3)
    (b: MaxI3, a: MaxU8)

    (i12 i16)
    (-0x0800 ..= 0x07ff)
    (a: MaxU8, b: MaxI4)
    (b: MaxI4, a: MaxU8)

    (i13 i16)
    (-0x1000 ..= 0x0fff)
    (a: MaxU8, b: MaxI5)
    (b: MaxI5, a: MaxU8)

    (i14 i16)
    (-0x2000 ..= 0x1fff)
    (a: MaxU8, b: MaxI6)
    (b: MaxI6, a: MaxU8)

    (i15 i16)
    (-0x4000 ..= 0x3fff)
    (a: MaxU8, b: MaxI7)
    (b: MaxI7, a: MaxU8)

    // (i16 i16)
    // (-0x8000 ..= 0x7fff)
    // (a: MaxU8, b: MaxI8)
    // (b: MaxI8, a: MaxU8)

    (i17 i32)
    (-0x0001_0000 ..= 0x0000_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI1, d: MaxI1)
    (d: MaxI1, c: MaxI1, b: MaxU8, a: MaxU8)

    (i18 i32)
    (-0x0002_0000 ..= 0x0001_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI2, d: MaxI1)
    (d: MaxI1, c: MaxI2, b: MaxU8, a: MaxU8)

    (i19 i32)
    (-0x0004_0000 ..= 0x0003_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI3, d: MaxI1)
    (d: MaxI1, c: MaxI3, b: MaxU8, a: MaxU8)

    (i20 i32)
    (-0x0008_0000 ..= 0x0007_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI4, d: MaxI1)
    (d: MaxI1, c: MaxI4, b: MaxU8, a: MaxU8)

    (i21 i32)
    (-0x0010_0000 ..= 0x000f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI5, d: MaxI1)
    (d: MaxI1, c: MaxI5, b: MaxU8, a: MaxU8)

    (i22 i32)
    (-0x0020_0000 ..= 0x001f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI6, d: MaxI1)
    (d: MaxI1, c: MaxI6, b: MaxU8, a: MaxU8)

    (i23 i32)
    (-0x0040_0000 ..= 0x003f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI7, d: MaxI1)
    (d: MaxI1, c: MaxI7, b: MaxU8, a: MaxU8)

    (i24 i32)
    (-0x0080_0000 ..= 0x007f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxI8, d: MaxI1)
    (d: MaxI1, c: MaxU8, b: MaxU8, a: MaxU8)

    (i25 i32)
    (-0x0100_0000 ..= 0x00ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI1)
    (d: MaxI1, c: MaxU8, b: MaxU8, a: MaxU8)

    (i26 i32)
    (-0x0200_0000 ..= 0x01ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI2)
    (d: MaxI2, c: MaxU8, b: MaxU8, a: MaxU8)

    (i27 i32)
    (-0x0400_0000 ..= 0x03ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI3)
    (d: MaxI3, c: MaxU8, b: MaxU8, a: MaxU8)

    (i28 i32)
    (-0x0800_0000 ..= 0x07ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI4)
    (d: MaxI4, c: MaxU8, b: MaxU8, a: MaxU8)

    (i29 i32)
    (-0x1000_0000 ..= 0x0fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI5)
    (d: MaxI5, c: MaxU8, b: MaxU8, a: MaxU8)

    (i30 i32)
    (-0x2000_0000 ..= 0x1fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI6)
    (d: MaxI6, c: MaxU8, b: MaxU8, a: MaxU8)

    (i31 i32)
    (-0x4000_0000 ..= 0x3fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI7)
    (d: MaxI7, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i32 i32)
    // (-0x8000_0000 ..= 0x7fff_ffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxI8)
    // (d: MaxI8, c: MaxU8, b: MaxU8, a: MaxU8)
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
            return 'MaxI1'
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
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI1, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI1, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i34 i64)
    (-0x0000000200000000 ..= 0x00000001ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI2, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI2, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i35 i64)
    (-0x0000000400000000 ..= 0x00000003ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI3, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI3, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i36 i64)
    (-0x0000000800000000 ..= 0x00000007ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI4, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI4, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i37 i64)
    (-0x0000001000000000 ..= 0x0000000fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI5, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI5, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i38 i64)
    (-0x0000002000000000 ..= 0x0000001fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI6, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI6, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i39 i64)
    (-0x0000004000000000 ..= 0x0000003fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI7, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI7, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i40 i64)
    (-0x0000008000000000 ..= 0x0000007fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxI8, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxI8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i41 i64)
    (-0x0000010000000000 ..= 0x000000ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI1, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI1, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i42 i64)
    (-0x0000020000000000 ..= 0x000001ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI2, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI2, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i43 i64)
    (-0x0000040000000000 ..= 0x000003ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI3, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI3, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i44 i64)
    (-0x0000080000000000 ..= 0x000007ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI4, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI4, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i45 i64)
    (-0x0000100000000000 ..= 0x00000fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI5, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI5, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i46 i64)
    (-0x0000200000000000 ..= 0x00001fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI6, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI6, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i47 i64)
    (-0x0000400000000000 ..= 0x00003fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI7, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxI7, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i48 i64)
    (-0x0000800000000000 ..= 0x00007fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxI8, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i49 i64)
    (-0x0001000000000000 ..= 0x0000ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI1, h: MaxI1)
    (h: MaxI1, g: MaxI1, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i50 i64)
    (-0x0002000000000000 ..= 0x0001ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI2, h: MaxI1)
    (h: MaxI1, g: MaxI2, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i51 i64)
    (-0x0004000000000000 ..= 0x0003ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI3, h: MaxI1)
    (h: MaxI1, g: MaxI3, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i52 i64)
    (-0x0008000000000000 ..= 0x0007ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI4, h: MaxI1)
    (h: MaxI1, g: MaxI4, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i53 i64)
    (-0x0010000000000000 ..= 0x000fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI5, h: MaxI1)
    (h: MaxI1, g: MaxI5, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i54 i64)
    (-0x0020000000000000 ..= 0x001fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI6, h: MaxI1)
    (h: MaxI1, g: MaxI6, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i55 i64)
    (-0x0040000000000000 ..= 0x003fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI7, h: MaxI1)
    (h: MaxI1, g: MaxI7, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i56 i64)
    (-0x0080000000000000 ..= 0x007fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxI8, h: MaxI1)
    (h: MaxI1, g: MaxI8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i57 i64)
    (-0x0100000000000000 ..= 0x00ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI1)
    (h: MaxI1, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i58 i64)
    (-0x0200000000000000 ..= 0x01ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI2)
    (h: MaxI2, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i59 i64)
    (-0x0400000000000000 ..= 0x03ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI3)
    (h: MaxI3, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i60 i64)
    (-0x0800000000000000 ..= 0x07ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI4)
    (h: MaxI4, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i61 i64)
    (-0x1000000000000000 ..= 0x0fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI5)
    (h: MaxI5, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i62 i64)
    (-0x2000000000000000 ..= 0x1fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI6)
    (h: MaxI6, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i63 i64)
    (-0x4000000000000000 ..= 0x3fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI7)
    (h: MaxI7, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i64 i64)
    // (-0x8000000000000000 ..= 0x7fffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxI8)
    // (h: MaxI8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

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
            return 'MaxI1'
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
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI1, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI1, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i66 i128)
    (-0x00000000000000020000000000000000 ..= 0x0000000000000001ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI2, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI2, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i67 i128)
    (-0x00000000000000040000000000000000 ..= 0x0000000000000003ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI3, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI3, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i68 i128)
    (-0x00000000000000080000000000000000 ..= 0x0000000000000007ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI4, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI4, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i69 i128)
    (-0x00000000000000100000000000000000 ..= 0x000000000000000fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI5, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI5, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i70 i128)
    (-0x00000000000000200000000000000000 ..= 0x000000000000001fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI6, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI6, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i71 i128)
    (-0x00000000000000400000000000000000 ..= 0x000000000000003fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI7, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI7, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i72 i128)
    (-0x00000000000000800000000000000000 ..= 0x000000000000007fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxI8, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxI8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i73 i128)
    (-0x00000000000001000000000000000000 ..= 0x00000000000000ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI1, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI1, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i74 i128)
    (-0x00000000000002000000000000000000 ..= 0x00000000000001ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI2, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI2, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i75 i128)
    (-0x00000000000004000000000000000000 ..= 0x00000000000003ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI3, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI3, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i76 i128)
    (-0x00000000000008000000000000000000 ..= 0x00000000000007ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI4, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI4, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i77 i128)
    (-0x00000000000010000000000000000000 ..= 0x0000000000000fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI5, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI5, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i78 i128)
    (-0x00000000000020000000000000000000 ..= 0x0000000000001fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI6, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI6, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i79 i128)
    (-0x00000000000040000000000000000000 ..= 0x0000000000003fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI7, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI7, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i80 i128)
    (-0x00000000000080000000000000000000 ..= 0x0000000000007fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxI8, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxI8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i81 i128)
    (-0x00000000000100000000000000000000 ..= 0x000000000000ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI1, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI1, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i82 i128)
    (-0x00000000000200000000000000000000 ..= 0x000000000001ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI2, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI2, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i83 i128)
    (-0x00000000000400000000000000000000 ..= 0x000000000003ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI3, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI3, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i84 i128)
    (-0x00000000000800000000000000000000 ..= 0x000000000007ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI4, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI4, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i85 i128)
    (-0x00000000001000000000000000000000 ..= 0x00000000000fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI5, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI5, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i86 i128)
    (-0x00000000002000000000000000000000 ..= 0x00000000001fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI6, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI6, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i87 i128)
    (-0x00000000004000000000000000000000 ..= 0x00000000003fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI7, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI7, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i88 i128)
    (-0x00000000008000000000000000000000 ..= 0x00000000007fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxI8, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxI8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i89 i128)
    (-0x00000000010000000000000000000000 ..= 0x0000000000ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI1, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI1, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i90 i128)
    (-0x00000000020000000000000000000000 ..= 0x0000000001ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI2, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI2, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i91 i128)
    (-0x00000000040000000000000000000000 ..= 0x0000000003ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI3, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI3, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i92 i128)
    (-0x00000000080000000000000000000000 ..= 0x0000000007ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI4, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI4, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i93 i128)
    (-0x00000000100000000000000000000000 ..= 0x000000000fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI5, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI5, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i94 i128)
    (-0x00000000200000000000000000000000 ..= 0x000000001fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI6, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI6, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i95 i128)
    (-0x00000000400000000000000000000000 ..= 0x000000003fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI7, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI7, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i96 i128)
    (-0x00000000800000000000000000000000 ..= 0x000000007fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxI8, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxI8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i97 i128)
    (-0x00000001000000000000000000000000 ..= 0x00000000ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI1, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI1, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i98 i128)
    (-0x00000002000000000000000000000000 ..= 0x00000001ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI2, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI2, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i99 i128)
    (-0x00000004000000000000000000000000 ..= 0x00000003ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI3, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI3, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i100 i128)
    (-0x00000008000000000000000000000000 ..= 0x00000007ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI4, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI4, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i101 i128)
    (-0x00000010000000000000000000000000 ..= 0x0000000fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI5, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI5, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i102 i128)
    (-0x00000020000000000000000000000000 ..= 0x0000001fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI6, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI6, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i103 i128)
    (-0x00000040000000000000000000000000 ..= 0x0000003fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI7, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI7, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i104 i128)
    (-0x00000080000000000000000000000000 ..= 0x0000007fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxI8, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxI8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i105 i128)
    (-0x00000100000000000000000000000000 ..= 0x000000ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI1, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI1, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i106 i128)
    (-0x00000200000000000000000000000000 ..= 0x000001ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI2, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI2, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i107 i128)
    (-0x00000400000000000000000000000000 ..= 0x000003ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI3, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI3, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i108 i128)
    (-0x00000800000000000000000000000000 ..= 0x000007ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI4, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI4, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i109 i128)
    (-0x00001000000000000000000000000000 ..= 0x00000fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI5, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI5, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i110 i128)
    (-0x00002000000000000000000000000000 ..= 0x00001fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI6, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI6, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i111 i128)
    (-0x00004000000000000000000000000000 ..= 0x00003fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI7, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI7, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i112 i128)
    (-0x00008000000000000000000000000000 ..= 0x00007fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxI8, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxI8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i113 i128)
    (-0x00010000000000000000000000000000 ..= 0x0000ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI1, p: MaxI1)
    (p: MaxI1, o: MaxI1, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i114 i128)
    (-0x00020000000000000000000000000000 ..= 0x0001ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI2, p: MaxI1)
    (p: MaxI1, o: MaxI2, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i115 i128)
    (-0x00040000000000000000000000000000 ..= 0x0003ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI3, p: MaxI1)
    (p: MaxI1, o: MaxI3, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i116 i128)
    (-0x00080000000000000000000000000000 ..= 0x0007ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI4, p: MaxI1)
    (p: MaxI1, o: MaxI4, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i117 i128)
    (-0x00100000000000000000000000000000 ..= 0x000fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI5, p: MaxI1)
    (p: MaxI1, o: MaxI5, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i118 i128)
    (-0x00200000000000000000000000000000 ..= 0x001fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI6, p: MaxI1)
    (p: MaxI1, o: MaxI6, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i119 i128)
    (-0x00400000000000000000000000000000 ..= 0x003fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI7, p: MaxI1)
    (p: MaxI1, o: MaxI7, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i120 i128)
    (-0x00800000000000000000000000000000 ..= 0x007fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxI8, p: MaxI1)
    (p: MaxI1, o: MaxI8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i121 i128)
    (-0x01000000000000000000000000000000 ..= 0x00ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI1)
    (p: MaxI1, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i122 i128)
    (-0x02000000000000000000000000000000 ..= 0x01ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI2)
    (p: MaxI2, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i123 i128)
    (-0x04000000000000000000000000000000 ..= 0x03ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI3)
    (p: MaxI3, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i124 i128)
    (-0x08000000000000000000000000000000 ..= 0x07ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI4)
    (p: MaxI4, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i125 i128)
    (-0x10000000000000000000000000000000 ..= 0x0fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI5)
    (p: MaxI5, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i126 i128)
    (-0x20000000000000000000000000000000 ..= 0x1fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI6)
    (p: MaxI6, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (i127 i128)
    (-0x40000000000000000000000000000000 ..= 0x3fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI7)
    (p: MaxI7, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (i128 i128)
    // (-0x80000000000000000000000000000000 ..= 0x7fffffffffffffffffffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxI8)
    // (p: MaxI8, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)
}
