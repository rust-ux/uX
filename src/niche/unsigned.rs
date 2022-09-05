use super::new_type;
use super::unsigned_bytes::*;

new_type! {
    (u1 u8)
    (0 ..= 0x01)
    (a: Max1)
    (a: Max1)

    (u2 u8)
    (0 ..= 0x03)
    (a: Max2)
    (a: Max2)

    (u3 u8)
    (0 ..= 0x07)
    (a: Max3)
    (a: Max3)

    (u4 u8)
    (0 ..= 0x0f)
    (a: Max4)
    (a: Max4)

    (u5 u8)
    (0 ..= 0x1f)
    (a: Max5)
    (a: Max5)

    (u6 u8)
    (0 ..= 0x3f)
    (a: Max6)
    (a: Max6)

    (u7 u8)
    (0 ..= 0x7f)
    (a: Max7)
    (a: Max7)

    // (u8 u8)
    // (0 ..= 0xff)
    // (a: Max8)
    // (a: Max8)

    (u9 u16)
    (0 ..= 0x01ff)
    (a: Max8, b: Max1)
    (b: Max1, a: Max8)

    (u10 u16)
    (0 ..= 0x03ff)
    (a: Max8, b: Max2)
    (b: Max2, a: Max8)

    (u11 u16)
    (0 ..= 0x07ff)
    (a: Max8, b: Max3)
    (b: Max3, a: Max8)

    (u12 u16)
    (0 ..= 0x0fff)
    (a: Max8, b: Max4)
    (b: Max4, a: Max8)

    (u13 u16)
    (0 ..= 0x1fff)
    (a: Max8, b: Max5)
    (b: Max5, a: Max8)

    (u14 u16)
    (0 ..= 0x3fff)
    (a: Max8, b: Max6)
    (b: Max6, a: Max8)

    (u15 u16)
    (0 ..= 0x7fff)
    (a: Max8, b: Max7)
    (b: Max7, a: Max8)

    // (u16 u16)
    // (0 ..= 0xffff)
    // (a: Max8, b: Max8)
    // (b: Max8, a: Max8)

    (u17 u32)
    (0 ..= 0x0001_ffff)
    (a: Max8, b: Max8, c: Max1, d: Max0)
    (d: Max0, c: Max1, b: Max8, a: Max8)

    (u18 u32)
    (0 ..= 0x0003_ffff)
    (a: Max8, b: Max8, c: Max2, d: Max0)
    (d: Max0, c: Max2, b: Max8, a: Max8)

    (u19 u32)
    (0 ..= 0x0007_ffff)
    (a: Max8, b: Max8, c: Max3, d: Max0)
    (d: Max0, c: Max3, b: Max8, a: Max8)

    (u20 u32)
    (0 ..= 0x000f_ffff)
    (a: Max8, b: Max8, c: Max4, d: Max0)
    (d: Max0, c: Max4, b: Max8, a: Max8)

    (u21 u32)
    (0 ..= 0x001f_ffff)
    (a: Max8, b: Max8, c: Max5, d: Max0)
    (d: Max0, c: Max5, b: Max8, a: Max8)

    (u22 u32)
    (0 ..= 0x003f_ffff)
    (a: Max8, b: Max8, c: Max6, d: Max0)
    (d: Max0, c: Max6, b: Max8, a: Max8)

    (u23 u32)
    (0 ..= 0x007f_ffff)
    (a: Max8, b: Max8, c: Max7, d: Max0)
    (d: Max1, c: Max7, b: Max8, a: Max8)

    (u24 u32)
    (0 ..= 0x00ff_ffff)
    (a: Max8, b: Max8, c: Max8, d: Max0)
    (d: Max0, c: Max8, b: Max8, a: Max8)

    (u25 u32)
    (0 ..= 0x01ff_ffff)
    (a: Max8, b: Max8, c: Max8, d: Max1)
    (d: Max1, c: Max8, b: Max8, a: Max8)

    (u26 u32)
    (0 ..= 0x03ff_ffff)
    (a: Max8, b: Max8, c: Max8, d: Max2)
    (d: Max2, c: Max8, b: Max8, a: Max8)

    (u27 u32)
    (0 ..= 0x07ff_ffff)
    (a: Max8, b: Max8, c: Max8, d: Max3)
    (d: Max3, c: Max8, b: Max8, a: Max8)

    (u28 u32)
    (0 ..= 0x0fff_ffff)
    (a: Max8, b: Max8, c: Max8, d: Max4)
    (d: Max4, c: Max8, b: Max8, a: Max8)

    (u29 u32)
    (0 ..= 0x1fff_ffff)
    (a: Max8, b: Max8, c: Max8, d: Max5)
    (d: Max5, c: Max8, b: Max8, a: Max8)

    (u30 u32)
    (0 ..= 0x3fff_ffff)
    (a: Max8, b: Max8, c: Max8, d: Max6)
    (d: Max6, c: Max8, b: Max8, a: Max8)

    (u31 u32)
    (0 ..= 0x7fff_ffff)
    (a: Max8, b: Max8, c: Max8, d: Max7)
    (d: Max7, c: Max8, b: Max8, a: Max8)

    // (u32 u32)
    // (0 ..= 0xffff_ffff)
    // (a: Max8, b: Max8, c: Max8, d: Max8)
    // (d: Max8, c: Max8, b: Max8, a: Max8)

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
    print(f'    (a: Max8, b: Max8, c: Max8, d: Max8, e: {M(e, f)}, f: {M(f, g)}, g: {M(g, h)}, h: {M(h, 0)})')
    print(f'    (h: {M(h, 0)}, g: {M(g, h)}, f: {M(f, g)}, e: {M(e, f)}, d: Max8, c: Max8, b: Max8, a: Max8)')
    print()
*/

    (u33 u64)
    (0 ..= 0x00000001ffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max1, f: Max0, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max0, e: Max1, d: Max8, c: Max8, b: Max8, a: Max8)

    (u34 u64)
    (0 ..= 0x00000003ffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max2, f: Max0, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max0, e: Max2, d: Max8, c: Max8, b: Max8, a: Max8)

    (u35 u64)
    (0 ..= 0x00000007ffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max3, f: Max0, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max0, e: Max3, d: Max8, c: Max8, b: Max8, a: Max8)

    (u36 u64)
    (0 ..= 0x0000000fffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max4, f: Max0, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max0, e: Max4, d: Max8, c: Max8, b: Max8, a: Max8)

    (u37 u64)
    (0 ..= 0x0000001fffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max5, f: Max0, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max0, e: Max5, d: Max8, c: Max8, b: Max8, a: Max8)

    (u38 u64)
    (0 ..= 0x0000003fffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max6, f: Max0, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max0, e: Max6, d: Max8, c: Max8, b: Max8, a: Max8)

    (u39 u64)
    (0 ..= 0x0000007fffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max7, f: Max0, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max0, e: Max7, d: Max8, c: Max8, b: Max8, a: Max8)

    (u40 u64)
    (0 ..= 0x000000ffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max0, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max0, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u41 u64)
    (0 ..= 0x000001ffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max1, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max1, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u42 u64)
    (0 ..= 0x000003ffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max2, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max2, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u43 u64)
    (0 ..= 0x000007ffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max3, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max3, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u44 u64)
    (0 ..= 0x00000fffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max4, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max4, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u45 u64)
    (0 ..= 0x00001fffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max5, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max5, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u46 u64)
    (0 ..= 0x00003fffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max6, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max6, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u47 u64)
    (0 ..= 0x00007fffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max7, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max7, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u48 u64)
    (0 ..= 0x0000ffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max0, h: Max0)
    (h: Max0, g: Max0, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u49 u64)
    (0 ..= 0x0001ffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max1, h: Max0)
    (h: Max0, g: Max1, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u50 u64)
    (0 ..= 0x0003ffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max2, h: Max0)
    (h: Max0, g: Max2, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u51 u64)
    (0 ..= 0x0007ffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max3, h: Max0)
    (h: Max0, g: Max3, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u52 u64)
    (0 ..= 0x000fffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max4, h: Max0)
    (h: Max0, g: Max4, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u53 u64)
    (0 ..= 0x001fffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max5, h: Max0)
    (h: Max0, g: Max5, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u54 u64)
    (0 ..= 0x003fffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max6, h: Max0)
    (h: Max0, g: Max6, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u55 u64)
    (0 ..= 0x007fffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max7, h: Max0)
    (h: Max0, g: Max7, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u56 u64)
    (0 ..= 0x00ffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max0)
    (h: Max0, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u57 u64)
    (0 ..= 0x01ffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max1)
    (h: Max1, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u58 u64)
    (0 ..= 0x03ffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max2)
    (h: Max2, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u59 u64)
    (0 ..= 0x07ffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max3)
    (h: Max3, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u60 u64)
    (0 ..= 0x0fffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max4)
    (h: Max4, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u61 u64)
    (0 ..= 0x1fffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max5)
    (h: Max5, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u62 u64)
    (0 ..= 0x3fffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max6)
    (h: Max6, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u63 u64)
    (0 ..= 0x7fffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max7)
    (h: Max7, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    // (u64 u64)
    // (0 ..= 0xffffffffffffffff)
    // (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8)
    // (h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

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
    print(f'    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: {M(i, j)}, j: {M(j, k)}, k: {M(k, l)}, l: {M(l, m)}, m: {M(m, n)}, n: {M(n, o)}, o: {M(o, p)}, p: Max{p or 1})')
    print(f'    (p: Max{p or 1}, o: {M(o, p)}, n: {M(n, o)}, m: {M(m, n)}, l: {M(l, m)}, k: {M(k, l)}, j: {M(j, k)}, i: {M(i, j)}, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)')
    print()

*/

    (u65 u128)
    (0 ..= 0x0000000000000001ffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max1, j: Max0, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max0, i: Max1, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u66 u128)
    (0 ..= 0x0000000000000003ffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max2, j: Max0, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max0, i: Max2, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u67 u128)
    (0 ..= 0x0000000000000007ffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max3, j: Max0, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max0, i: Max3, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u68 u128)
    (0 ..= 0x000000000000000fffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max4, j: Max0, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max0, i: Max4, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u69 u128)
    (0 ..= 0x000000000000001fffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max5, j: Max0, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max0, i: Max5, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u70 u128)
    (0 ..= 0x000000000000003fffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max6, j: Max0, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max0, i: Max6, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u71 u128)
    (0 ..= 0x000000000000007fffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max7, j: Max0, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max0, i: Max7, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u72 u128)
    (0 ..= 0x00000000000000ffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max0, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max0, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u73 u128)
    (0 ..= 0x00000000000001ffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max1, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max1, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u74 u128)
    (0 ..= 0x00000000000003ffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max2, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max2, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u75 u128)
    (0 ..= 0x00000000000007ffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max3, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max3, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u76 u128)
    (0 ..= 0x0000000000000fffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max4, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max4, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u77 u128)
    (0 ..= 0x0000000000001fffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max5, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max5, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u78 u128)
    (0 ..= 0x0000000000003fffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max6, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max6, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u79 u128)
    (0 ..= 0x0000000000007fffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max7, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max7, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u80 u128)
    (0 ..= 0x000000000000ffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max0, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max0, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u81 u128)
    (0 ..= 0x000000000001ffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max1, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max1, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u82 u128)
    (0 ..= 0x000000000003ffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max2, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max2, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u83 u128)
    (0 ..= 0x000000000007ffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max3, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max3, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u84 u128)
    (0 ..= 0x00000000000fffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max4, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max4, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u85 u128)
    (0 ..= 0x00000000001fffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max5, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max5, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u86 u128)
    (0 ..= 0x00000000003fffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max6, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max6, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u87 u128)
    (0 ..= 0x00000000007fffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max7, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max7, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u88 u128)
    (0 ..= 0x0000000000ffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max0, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max0, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u89 u128)
    (0 ..= 0x0000000001ffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max1, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max1, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u90 u128)
    (0 ..= 0x0000000003ffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max2, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max2, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u91 u128)
    (0 ..= 0x0000000007ffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max3, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max3, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u92 u128)
    (0 ..= 0x000000000fffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max4, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max4, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u93 u128)
    (0 ..= 0x000000001fffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max5, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max5, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u94 u128)
    (0 ..= 0x000000003fffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max6, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max6, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u95 u128)
    (0 ..= 0x000000007fffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max7, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max7, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u96 u128)
    (0 ..= 0x00000000ffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max0, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max0, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u97 u128)
    (0 ..= 0x00000001ffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max1, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max1, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u98 u128)
    (0 ..= 0x00000003ffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max2, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max2, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u99 u128)
    (0 ..= 0x00000007ffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max3, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max3, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u100 u128)
    (0 ..= 0x0000000fffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max4, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max4, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u101 u128)
    (0 ..= 0x0000001fffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max5, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max5, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u102 u128)
    (0 ..= 0x0000003fffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max6, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max6, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u103 u128)
    (0 ..= 0x0000007fffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max7, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max7, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u104 u128)
    (0 ..= 0x000000ffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max0, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max0, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u105 u128)
    (0 ..= 0x000001ffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max1, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max1, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u106 u128)
    (0 ..= 0x000003ffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max2, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max2, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u107 u128)
    (0 ..= 0x000007ffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max3, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max3, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u108 u128)
    (0 ..= 0x00000fffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max4, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max4, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u109 u128)
    (0 ..= 0x00001fffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max5, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max5, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u110 u128)
    (0 ..= 0x00003fffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max6, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max6, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u111 u128)
    (0 ..= 0x00007fffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max7, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max7, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u112 u128)
    (0 ..= 0x0000ffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max0, p: Max1)
    (p: Max1, o: Max0, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u113 u128)
    (0 ..= 0x0001ffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max1, p: Max1)
    (p: Max1, o: Max1, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u114 u128)
    (0 ..= 0x0003ffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max2, p: Max1)
    (p: Max1, o: Max2, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u115 u128)
    (0 ..= 0x0007ffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max3, p: Max1)
    (p: Max1, o: Max3, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u116 u128)
    (0 ..= 0x000fffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max4, p: Max1)
    (p: Max1, o: Max4, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u117 u128)
    (0 ..= 0x001fffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max5, p: Max1)
    (p: Max1, o: Max5, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u118 u128)
    (0 ..= 0x003fffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max6, p: Max1)
    (p: Max1, o: Max6, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u119 u128)
    (0 ..= 0x007fffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max7, p: Max1)
    (p: Max1, o: Max7, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u120 u128)
    (0 ..= 0x00ffffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max1)
    (p: Max1, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u121 u128)
    (0 ..= 0x01ffffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max1)
    (p: Max1, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u122 u128)
    (0 ..= 0x03ffffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max2)
    (p: Max2, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u123 u128)
    (0 ..= 0x07ffffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max3)
    (p: Max3, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u124 u128)
    (0 ..= 0x0fffffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max4)
    (p: Max4, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u125 u128)
    (0 ..= 0x1fffffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max5)
    (p: Max5, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u126 u128)
    (0 ..= 0x3fffffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max6)
    (p: Max6, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    (u127 u128)
    (0 ..= 0x7fffffffffffffffffffffffffffffff)
    (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max7)
    (p: Max7, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)

    // (u128 u128)
    // (0 ..= 0xffffffffffffffffffffffffffffffff)
    // (a: Max8, b: Max8, c: Max8, d: Max8, e: Max8, f: Max8, g: Max8, h: Max8, i: Max8, j: Max8, k: Max8, l: Max8, m: Max8, n: Max8, o: Max8, p: Max8)
    // (p: Max8, o: Max8, n: Max8, m: Max8, l: Max8, k: Max8, j: Max8, i: Max8, h: Max8, g: Max8, f: Max8, e: Max8, d: Max8, c: Max8, b: Max8, a: Max8)
}
