fn calc_ext_euclid(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);

    while r1 != 0 {
        let q = r0 / r1;

        let r2 = r0 % r1;
        r0 = r1;
        r1 = r2;

        let s2 = s0 - q * s1;
        s0 = s1;
        s1 = s2;

        let t2 = t0 - q * t1;
        t0 = t1;
        t1 = t2;
    }

    (r0, s0, t0)
}

pub fn ext_euclid(a: i32, b: i32) -> (i32, i32, i32) {
    let (gcd, s, t) = calc_ext_euclid(a, b);

    (gcd, s, t)
}
