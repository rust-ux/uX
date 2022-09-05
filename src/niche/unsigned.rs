use super::new_type;
use super::unsigned_bytes::*;

new_type! {
    (u1 u8)
    (0 ..= 0x01)
    (a: MaxU1)
    (a: MaxU1)

    (u2 u8)
    (0 ..= 0x03)
    (a: MaxU2)
    (a: MaxU2)

    (u3 u8)
    (0 ..= 0x07)
    (a: MaxU3)
    (a: MaxU3)

    (u4 u8)
    (0 ..= 0x0f)
    (a: MaxU4)
    (a: MaxU4)

    (u5 u8)
    (0 ..= 0x1f)
    (a: MaxU5)
    (a: MaxU5)

    (u6 u8)
    (0 ..= 0x3f)
    (a: MaxU6)
    (a: MaxU6)

    (u7 u8)
    (0 ..= 0x7f)
    (a: MaxU7)
    (a: MaxU7)

    // (u8 u8)
    // (0 ..= 0xff)
    // (a: MaxU8)
    // (a: MaxU8)

    (u9 u16)
    (0 ..= 0x01ff)
    (a: MaxU8, b: MaxU1)
    (b: MaxU1, a: MaxU8)

    (u10 u16)
    (0 ..= 0x03ff)
    (a: MaxU8, b: MaxU2)
    (b: MaxU2, a: MaxU8)

    (u11 u16)
    (0 ..= 0x07ff)
    (a: MaxU8, b: MaxU3)
    (b: MaxU3, a: MaxU8)

    (u12 u16)
    (0 ..= 0x0fff)
    (a: MaxU8, b: MaxU4)
    (b: MaxU4, a: MaxU8)

    (u13 u16)
    (0 ..= 0x1fff)
    (a: MaxU8, b: MaxU5)
    (b: MaxU5, a: MaxU8)

    (u14 u16)
    (0 ..= 0x3fff)
    (a: MaxU8, b: MaxU6)
    (b: MaxU6, a: MaxU8)

    (u15 u16)
    (0 ..= 0x7fff)
    (a: MaxU8, b: MaxU7)
    (b: MaxU7, a: MaxU8)

    // (u16 u16)
    // (0 ..= 0xffff)
    // (a: MaxU8, b: MaxU8)
    // (b: MaxU8, a: MaxU8)

    (u17 u32)
    (0 ..= 0x0001_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU1, d: MaxU0)
    (d: MaxU0, c: MaxU1, b: MaxU8, a: MaxU8)

    (u18 u32)
    (0 ..= 0x0003_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU2, d: MaxU0)
    (d: MaxU0, c: MaxU2, b: MaxU8, a: MaxU8)

    (u19 u32)
    (0 ..= 0x0007_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU3, d: MaxU0)
    (d: MaxU0, c: MaxU3, b: MaxU8, a: MaxU8)

    (u20 u32)
    (0 ..= 0x000f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU4, d: MaxU0)
    (d: MaxU0, c: MaxU4, b: MaxU8, a: MaxU8)

    (u21 u32)
    (0 ..= 0x001f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU5, d: MaxU0)
    (d: MaxU0, c: MaxU5, b: MaxU8, a: MaxU8)

    (u22 u32)
    (0 ..= 0x003f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU6, d: MaxU0)
    (d: MaxU0, c: MaxU6, b: MaxU8, a: MaxU8)

    (u23 u32)
    (0 ..= 0x007f_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU7, d: MaxU0)
    (d: MaxU1, c: MaxU7, b: MaxU8, a: MaxU8)

    (u24 u32)
    (0 ..= 0x00ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU0)
    (d: MaxU0, c: MaxU8, b: MaxU8, a: MaxU8)

    (u25 u32)
    (0 ..= 0x01ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU1)
    (d: MaxU1, c: MaxU8, b: MaxU8, a: MaxU8)

    (u26 u32)
    (0 ..= 0x03ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU2)
    (d: MaxU2, c: MaxU8, b: MaxU8, a: MaxU8)

    (u27 u32)
    (0 ..= 0x07ff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU3)
    (d: MaxU3, c: MaxU8, b: MaxU8, a: MaxU8)

    (u28 u32)
    (0 ..= 0x0fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU4)
    (d: MaxU4, c: MaxU8, b: MaxU8, a: MaxU8)

    (u29 u32)
    (0 ..= 0x1fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU5)
    (d: MaxU5, c: MaxU8, b: MaxU8, a: MaxU8)

    (u30 u32)
    (0 ..= 0x3fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU6)
    (d: MaxU6, c: MaxU8, b: MaxU8, a: MaxU8)

    (u31 u32)
    (0 ..= 0x7fff_ffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU7)
    (d: MaxU7, c: MaxU8, b: MaxU8, a: MaxU8)

    // (u32 u32)
    // (0 ..= 0xffff_ffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8)
    // (d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

/*
e = f = g = h = 0
for I in range(33, 65):
    if e < 8:
        e += 1
    elif f < 8:
        f += 1
    elif g < 8:
        g += 1
    elif h < 8:
        h += 1
    def M(i, j):
        return f'Max{i}'
    print(f'    (u{I} u64)')
    print(f'    (0 ..= 0x{2**I-1:016x})')
    print(f'    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: {M(e, f)}, f: {M(f, g)}, g: {M(g, h)}, h: {M(h, 0)})')
    print(f'    (h: {M(h, 0)}, g: {M(g, h)}, f: {M(f, g)}, e: {M(e, f)}, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)')
    print()
*/

    (u33 u64)
    (0 ..= 0x00000001ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU1, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU1, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u34 u64)
    (0 ..= 0x00000003ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU2, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU2, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u35 u64)
    (0 ..= 0x00000007ffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU3, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU3, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u36 u64)
    (0 ..= 0x0000000fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU4, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU4, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u37 u64)
    (0 ..= 0x0000001fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU5, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU5, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u38 u64)
    (0 ..= 0x0000003fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU6, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU6, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u39 u64)
    (0 ..= 0x0000007fffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU7, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU7, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u40 u64)
    (0 ..= 0x000000ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU0, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU0, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u41 u64)
    (0 ..= 0x000001ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU1, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU1, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u42 u64)
    (0 ..= 0x000003ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU2, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU2, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u43 u64)
    (0 ..= 0x000007ffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU3, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU3, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u44 u64)
    (0 ..= 0x00000fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU4, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU4, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u45 u64)
    (0 ..= 0x00001fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU5, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU5, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u46 u64)
    (0 ..= 0x00003fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU6, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU6, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u47 u64)
    (0 ..= 0x00007fffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU7, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU7, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u48 u64)
    (0 ..= 0x0000ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU0, h: MaxU0)
    (h: MaxU0, g: MaxU0, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u49 u64)
    (0 ..= 0x0001ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU1, h: MaxU0)
    (h: MaxU0, g: MaxU1, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u50 u64)
    (0 ..= 0x0003ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU2, h: MaxU0)
    (h: MaxU0, g: MaxU2, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u51 u64)
    (0 ..= 0x0007ffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU3, h: MaxU0)
    (h: MaxU0, g: MaxU3, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u52 u64)
    (0 ..= 0x000fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU4, h: MaxU0)
    (h: MaxU0, g: MaxU4, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u53 u64)
    (0 ..= 0x001fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU5, h: MaxU0)
    (h: MaxU0, g: MaxU5, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u54 u64)
    (0 ..= 0x003fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU6, h: MaxU0)
    (h: MaxU0, g: MaxU6, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u55 u64)
    (0 ..= 0x007fffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU7, h: MaxU0)
    (h: MaxU0, g: MaxU7, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u56 u64)
    (0 ..= 0x00ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU0)
    (h: MaxU0, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u57 u64)
    (0 ..= 0x01ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU1)
    (h: MaxU1, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u58 u64)
    (0 ..= 0x03ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU2)
    (h: MaxU2, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u59 u64)
    (0 ..= 0x07ffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU3)
    (h: MaxU3, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u60 u64)
    (0 ..= 0x0fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU4)
    (h: MaxU4, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u61 u64)
    (0 ..= 0x1fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU5)
    (h: MaxU5, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u62 u64)
    (0 ..= 0x3fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU6)
    (h: MaxU6, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u63 u64)
    (0 ..= 0x7fffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU7)
    (h: MaxU7, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (u64 u64)
    // (0 ..= 0xffffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8)
    // (h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

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
        return f'Max{i}'
    print(f'    (u{I} u128)')
    print(f'    (0 ..= 0x{2**I-1:032x})')
    print(f'    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: {M(i, j)}, j: {M(j, k)}, k: {M(k, l)}, l: {M(l, m)}, m: {M(m, n)}, n: {M(n, o)}, o: {M(o, p)}, p: Max{p or 1})')
    print(f'    (p: Max{p or 1}, o: {M(o, p)}, n: {M(n, o)}, m: {M(m, n)}, l: {M(l, m)}, k: {M(k, l)}, j: {M(j, k)}, i: {M(i, j)}, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)')
    print()

*/

    (u65 u128)
    (0 ..= 0x0000000000000001ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU1, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU1, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u66 u128)
    (0 ..= 0x0000000000000003ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU2, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU2, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u67 u128)
    (0 ..= 0x0000000000000007ffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU3, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU3, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u68 u128)
    (0 ..= 0x000000000000000fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU4, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU4, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u69 u128)
    (0 ..= 0x000000000000001fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU5, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU5, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u70 u128)
    (0 ..= 0x000000000000003fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU6, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU6, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u71 u128)
    (0 ..= 0x000000000000007fffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU7, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU7, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u72 u128)
    (0 ..= 0x00000000000000ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU0, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU0, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u73 u128)
    (0 ..= 0x00000000000001ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU1, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU1, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u74 u128)
    (0 ..= 0x00000000000003ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU2, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU2, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u75 u128)
    (0 ..= 0x00000000000007ffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU3, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU3, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u76 u128)
    (0 ..= 0x0000000000000fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU4, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU4, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u77 u128)
    (0 ..= 0x0000000000001fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU5, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU5, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u78 u128)
    (0 ..= 0x0000000000003fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU6, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU6, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u79 u128)
    (0 ..= 0x0000000000007fffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU7, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU7, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u80 u128)
    (0 ..= 0x000000000000ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU0, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU0, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u81 u128)
    (0 ..= 0x000000000001ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU1, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU1, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u82 u128)
    (0 ..= 0x000000000003ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU2, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU2, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u83 u128)
    (0 ..= 0x000000000007ffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU3, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU3, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u84 u128)
    (0 ..= 0x00000000000fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU4, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU4, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u85 u128)
    (0 ..= 0x00000000001fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU5, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU5, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u86 u128)
    (0 ..= 0x00000000003fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU6, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU6, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u87 u128)
    (0 ..= 0x00000000007fffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU7, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU7, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u88 u128)
    (0 ..= 0x0000000000ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU0, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU0, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u89 u128)
    (0 ..= 0x0000000001ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU1, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU1, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u90 u128)
    (0 ..= 0x0000000003ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU2, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU2, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u91 u128)
    (0 ..= 0x0000000007ffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU3, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU3, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u92 u128)
    (0 ..= 0x000000000fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU4, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU4, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u93 u128)
    (0 ..= 0x000000001fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU5, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU5, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u94 u128)
    (0 ..= 0x000000003fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU6, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU6, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u95 u128)
    (0 ..= 0x000000007fffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU7, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU7, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u96 u128)
    (0 ..= 0x00000000ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU0, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU0, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u97 u128)
    (0 ..= 0x00000001ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU1, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU1, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u98 u128)
    (0 ..= 0x00000003ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU2, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU2, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u99 u128)
    (0 ..= 0x00000007ffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU3, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU3, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u100 u128)
    (0 ..= 0x0000000fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU4, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU4, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u101 u128)
    (0 ..= 0x0000001fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU5, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU5, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u102 u128)
    (0 ..= 0x0000003fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU6, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU6, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u103 u128)
    (0 ..= 0x0000007fffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU7, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU7, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u104 u128)
    (0 ..= 0x000000ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU0, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU0, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u105 u128)
    (0 ..= 0x000001ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU1, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU1, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u106 u128)
    (0 ..= 0x000003ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU2, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU2, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u107 u128)
    (0 ..= 0x000007ffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU3, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU3, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u108 u128)
    (0 ..= 0x00000fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU4, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU4, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u109 u128)
    (0 ..= 0x00001fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU5, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU5, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u110 u128)
    (0 ..= 0x00003fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU6, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU6, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u111 u128)
    (0 ..= 0x00007fffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU7, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU7, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u112 u128)
    (0 ..= 0x0000ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU0, p: MaxU1)
    (p: MaxU1, o: MaxU0, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u113 u128)
    (0 ..= 0x0001ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU1, p: MaxU1)
    (p: MaxU1, o: MaxU1, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u114 u128)
    (0 ..= 0x0003ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU2, p: MaxU1)
    (p: MaxU1, o: MaxU2, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u115 u128)
    (0 ..= 0x0007ffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU3, p: MaxU1)
    (p: MaxU1, o: MaxU3, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u116 u128)
    (0 ..= 0x000fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU4, p: MaxU1)
    (p: MaxU1, o: MaxU4, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u117 u128)
    (0 ..= 0x001fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU5, p: MaxU1)
    (p: MaxU1, o: MaxU5, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u118 u128)
    (0 ..= 0x003fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU6, p: MaxU1)
    (p: MaxU1, o: MaxU6, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u119 u128)
    (0 ..= 0x007fffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU7, p: MaxU1)
    (p: MaxU1, o: MaxU7, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u120 u128)
    (0 ..= 0x00ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU1)
    (p: MaxU1, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u121 u128)
    (0 ..= 0x01ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU1)
    (p: MaxU1, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u122 u128)
    (0 ..= 0x03ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU2)
    (p: MaxU2, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u123 u128)
    (0 ..= 0x07ffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU3)
    (p: MaxU3, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u124 u128)
    (0 ..= 0x0fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU4)
    (p: MaxU4, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u125 u128)
    (0 ..= 0x1fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU5)
    (p: MaxU5, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u126 u128)
    (0 ..= 0x3fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU6)
    (p: MaxU6, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    (u127 u128)
    (0 ..= 0x7fffffffffffffffffffffffffffffff)
    (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU7)
    (p: MaxU7, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)

    // (u128 u128)
    // (0 ..= 0xffffffffffffffffffffffffffffffff)
    // (a: MaxU8, b: MaxU8, c: MaxU8, d: MaxU8, e: MaxU8, f: MaxU8, g: MaxU8, h: MaxU8, i: MaxU8, j: MaxU8, k: MaxU8, l: MaxU8, m: MaxU8, n: MaxU8, o: MaxU8, p: MaxU8)
    // (p: MaxU8, o: MaxU8, n: MaxU8, m: MaxU8, l: MaxU8, k: MaxU8, j: MaxU8, i: MaxU8, h: MaxU8, g: MaxU8, f: MaxU8, e: MaxU8, d: MaxU8, c: MaxU8, b: MaxU8, a: MaxU8)
}
