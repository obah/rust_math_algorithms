// implementation 1 by me
// fn ext_euclid(a: i8, b: i8) -> (i8, i8, i8) {
//     if a < b {
//         panic!("A lesser then B")
//     }

//     let mut k = 2; //counter

//     let mut r: Vec<i8> = vec![a, b];
//     let mut s: Vec<i8> = vec![1, 0];
//     let mut t: Vec<i8> = vec![0, 1];
//     let mut q: Vec<i8> = vec![];

//     while r[k - 1] != 0 {
//         let q_new = r[k - 2] / r[k - 1];
//         q.push(q_new);

//         let r_new = r[k - 2] % r[k - 1];
//         r.push(r_new);

//         let s_new = s[k - 2] - (q[k - 2] * s[k - 1]);
//         s.push(s_new);

//         let t_new = t[k - 2] - (q[k - 2] * t[k - 1]);
//         t.push(t_new);

//         k += 1;
//     }

//     let gcd = r[k - 2];
//     let selected_s = s[k - 2];
//     let selected_t = t[k - 2];

//     let tester = (selected_s * a) + (selected_t * b);

//     assert_eq!(gcd, tester, "gcd test failed");

//     (gcd, selected_s, selected_t)
// }

// implementation 2 by chatGPT
fn ext_euclid(a: i8, b: i8) -> (i8, i8, i8) {
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

fn main() {
    let a = 12;
    let b = 5;

    let (gcd, s, t) = ext_euclid(a, b);

    // println!(
    //     "ext_euclid of {} and {} is => gcd: {}, s: {}, t: {}",
    //     a, b, gcd, s, t
    // );

    println!("gcd({}, {}) = {}", a, b, gcd);
    println!("s = {}, t = {}", s, t);
    println!(
        "Verification: {} * {} + {} * {} = {}",
        s,
        a,
        t,
        b,
        s * a + t * b
    );
}
