#[cfg(target_arch = "x86")]
pub(crate) use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
pub(crate) use core::arch::x86_64::*;

union Raw {
    m128: __m128,
    f32x4: (f32, f32, f32, f32),
}

pub const Q: __m128 = unsafe {
    (Raw {
        f32x4: (2. / 17., 4. / 17., 10. / 17., 13. / 17.),
    })
    .m128
};

pub fn quat_to_mat3_simd(q: &__m128) -> [__m128; 3] {
    unsafe {
        let u = *q;

        let u2 = _mm_add_ps(u, u);
        let s = _mm_shuffle_ps(u, u, 0b11_11_11_11);

        // 2si 2sj 2sk 2ss
        let su = _mm_mul_ps(u2, s);

        // 2j  2k  2i  2s
        let p = _mm_shuffle_ps(u2, u2, 0b11_00_10_01);

        // k   i   j   s
        let q = _mm_shuffle_ps(u, u, 0b11_01_00_10);

        // 2jk 2ki 2ij 2ss
        let r = _mm_mul_ps(p, q);

        // sum:  2ji+2si  2ki+2sj  2ij+2sk  4ss
        let s = _mm_add_ps(r, su);

        // diff: 2ji-2si  2ki-2sj  2ij-2sk  0
        let d = _mm_sub_ps(r, su);

        // 2ii 2jj 2kk 2ss
        let uu = _mm_mul_ps(u2, u);

        // 2kk 2ii 2jj 2ss
        let r = _mm_shuffle_ps(uu, uu, 0b11_01_00_10);

        let uu = _mm_add_ps(r, uu);
        let v = _mm_sub_ps(_mm_set1_ps(1.), uu);

        // v2 s2 v3 s3  unpackhi
        let v2s2v3s3 = _mm_unpackhi_ps(v, s);
        // d1 d3 d0 d3  shuffle
        let d1d3d0d3 = _mm_shuffle_ps(d, d, 0b11_00_11_01);
        // v2 s2 d1 d3  shuffle
        let c0 = _mm_shuffle_ps(v2s2v3s3, d1d3d0d3, 0b01_00_01_00);

        // s0 v0 s1 v1  unpacklo
        let s0v0s1v1 = _mm_unpacklo_ps(s, v);
        // v0 s0 d2 d3  shuffle
        let v0s0d2d3 = _mm_shuffle_ps(s0v0s1v1, d, 0b11_10_00_01);
        // d2 v0 s0 d3  shuffle
        let c1 = _mm_shuffle_ps(v0s0d2d3, v0s0d2d3, 0b11_01_00_10);

        // s1 d0 v1 d3  unpackhi
        let c2 = _mm_unpackhi_ps(s0v0s1v1, d1d3d0d3);

        [c0, c1, c2]
    }
}

pub fn quat_to_mat3_half_simd(q: &__m128) -> [__m128; 3] {
    unsafe {
        let u = *q;

        let u2 = _mm_add_ps(u, u);
        let s = _mm_shuffle_ps(u, u, 0b11_11_11_11);

        // 2si 2sj 2sk 2ss
        let su = _mm_mul_ps(u2, s);

        // 2j  2k  2i  2s
        let p = _mm_shuffle_ps(u2, u2, 0b11_00_10_01);

        // k   i   j   s
        let q = _mm_shuffle_ps(u, u, 0b11_01_00_10);

        // 2jk 2ki 2ij 2ss
        let r = _mm_mul_ps(p, q);

        // sum:  2ji+2si  2ki+2sj  2ij+2sk  4ss
        let s = _mm_add_ps(r, su);

        // diff: 2ji-2si  2ki-2sj  2ij-2sk  0
        let d = _mm_sub_ps(r, su);

        // 2ii 2jj 2kk 2ss
        let uu = _mm_mul_ps(u2, u);

        // 2kk 2ii 2jj 2ss
        let r = _mm_shuffle_ps(uu, uu, 0b11_01_00_10);

        let uu = _mm_add_ps(r, uu);
        let v = _mm_sub_ps(_mm_set1_ps(1.), uu);

        let s = &s as *const __m128 as *const f32;
        let d = &d as *const __m128 as *const f32;
        let v = &v as *const __m128 as *const f32;

        [
            _mm_setr_ps(*v.add(2), *s.add(2), *d.add(1), 0.),
            _mm_setr_ps(*d.add(2), *v.add(0), *s.add(0), 0.),
            _mm_setr_ps(*s.add(1), *d.add(0), *v.add(1), 0.),
        ]
    }
}

pub fn quat_to_mat3_portable(q: &__m128) -> [f32; 12] {
    let [i, j, k, s] = unsafe { *(q as *const _ as *const [f32; 4]) };

    let i2 = i + i;
    let j2 = j + j;
    let k2 = k + k;

    let ii = i2 * i;
    let jj = j2 * j;
    let kk = k2 * k;

    let si = i2 * s;
    let sj = j2 * s;
    let sk = k2 * s;

    let ij = i2 * j;
    let jk = j2 * k;
    let ki = k2 * i;

    [
        1. - (jj + kk),
        ij + sk,
        ki - sj,
        0.,
        ij - sk,
        1. - (ii + kk),
        jk + si,
        0.,
        ki + sj,
        jk - si,
        1. - (ii + jj),
        0.,
    ]
}

#[cfg(test)]
mod tests;
