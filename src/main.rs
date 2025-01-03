mod algorithms;

use algorithms::polynomial_long_division::{poly_long_division, Polynomial};
use algorithms::{chinese_remainder, extended_euclidean};

fn main() {
    //extended euclidean
    let a = 12;
    let b = 6;

    let (gcd, s, t) = extended_euclidean::ext_euclid(12, 6);

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

    //chinese remainder
    /*
    solve
    x cong 4 (mod 7)
    x cong 1 (mod 3)
    x cong 3 (mod 5)
    x cong 0 (mod 11)
    */
    let rh_values: Vec<usize> = vec![4, 1, 3, 0];
    let mods: Vec<usize> = vec![7, 3, 5, 11];

    chinese_remainder::chinese_remainder(&rh_values, &mods);

    //polynomial division
    let poly_a = Polynomial::new(vec![1.0, -3.0, 2.0]); // x^2 - 3x + 2
    let poly_b = Polynomial::new(vec![1.0, -1.0]); // x - 1

    let (quo, rem) = poly_long_division(poly_a, poly_b);
    println!("Quotient: {:?}", quo);
    println!("Remainder: {:?}", rem);

    let poly_c = Polynomial::new(vec![-9.0, 0.0, 0.0, 2.0, 0.0, 1.0]); // x^5 + 2x^3 - 9
    let poly_d = Polynomial::new(vec![-1.0, 4.0, 1.0]); // x^2 + 4x - 1

    let (quo, rem) = poly_long_division(poly_c, poly_d);
    println!("Quotient: {:?}", quo);
    println!("Remainder: {:?}", rem);
}
