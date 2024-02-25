use super::*;

/// Returns the dot product of two quaternions.
#[must_use]
pub(super) fn dot(a: &__m128, b: &__m128) -> f32 {
    unsafe {
        let s = _mm_mul_ps(*a, *b);
        let t = _mm_shuffle_ps(s, s, 0b10_11_00_01);
        let s = _mm_add_ps(s, t);
        let t = _mm_shuffle_ps(s, s, 0b01_00_11_10);
        _mm_cvtss_f32(_mm_add_ps(s, t))
    }
}

#[test]
fn correctness() {
    // make sure that `q` is a unit quaternion
    let d = dot(&Q, &Q);
    assert_eq!(d, 1.0);

    let portable = quat_to_mat3_portable(&Q);

    let half_simd = quat_to_mat3_half_simd(&Q);
    let half_simd = unsafe { *(&half_simd as *const _ as *const [f32; 12]) };

    let simd = quat_to_mat3_simd(&Q);
    let simd = unsafe { *(&simd as *const _ as *const [f32; 12]) };

    assert_eq!(half_simd, portable);
    assert_eq!(half_simd, simd);
}
