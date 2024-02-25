fn main() {
    find_quadruples();
}

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
