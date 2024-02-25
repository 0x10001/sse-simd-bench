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

#[test]
fn find_quadruples() {
    let u = 20;
    for a in 1..u {
        for b in (a + 1)..u {
            for c in (b + 1)..u {
                for d in (c + 1)..u {
                    let t = a * a + b * b + c * c + d * d;
                    if let Some(r) = perfect(t) {
                        println!(
                            "{a:?}. / {r:?}., {b:?}. / {r:?}., {c:?}. / {r:?}., {d:?}. / {r:?}."
                        );
                    }
                }
            }
        }
    }
}

fn perfect(n: usize) -> Option<usize> {
    let mut lo = 1;
    let mut hi = 1 << 12;

    while lo < hi {
        let mid = lo + (hi - lo >> 1);
        let m2 = mid * mid;
        if m2 == n {
            return Some(mid);
        }
        if m2 < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    if lo * lo == n {
        Some(lo)
    } else {
        None
    }
}
