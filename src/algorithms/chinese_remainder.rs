use crate::algorithms::extended_euclidean::ext_euclid;

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn check_coprime(mods: &[usize]) -> bool {
    let mut is_coprime = true;

    for i in 0..mods.len() {
        for j in (i + 1)..mods.len() {
            if gcd(mods[i], mods[j]) != 1 {
                is_coprime = false;
            }
        }
    }

    is_coprime
}

fn solve_chinese_remainder(rh_values: &[usize], mods: &[usize]) -> usize {
    if !check_coprime(mods) {
        panic!("The mods arent coprime");
    }

    let mods_product: usize = mods.iter().product(); //N

    let mut x: usize = 0;

    for i in 0..(mods.len() - 1) {
        let n_i = mods_product / mods[i];

        let (_, s, _) = ext_euclid(n_i.try_into().unwrap(), mods[i].try_into().unwrap());

        let s: usize = s.try_into().unwrap();

        let x_prime = rh_values[i] * s * n_i as usize;

        x += x_prime;
    }

    x % mods_product
}

pub fn chinese_remainder(rh_values: &[usize], mods: &[usize]) -> usize {
    let remainder = solve_chinese_remainder(rh_values, mods);

    println!("Remainder for these congruences are {remainder}");

    remainder
}
