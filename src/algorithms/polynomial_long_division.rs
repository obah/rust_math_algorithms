use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Polynomial {
    coeffs: Vec<f64>,
}

impl Polynomial {
    pub fn new(coeffs: Vec<f64>) -> Self {
        Polynomial { coeffs }
    }

    fn trim(&mut self) {
        while self.coeffs.last() == Some(&0.0) {
            self.coeffs.pop();
        }
    }

    fn degree(&mut self) -> usize {
        self.trim();

        if self.coeffs.is_empty() {
            0
        } else {
            self.coeffs.len() - 1
        }
    }

    fn leading_coeff(&mut self) -> f64 {
        self.trim();

        *self.coeffs.last().unwrap_or(&0.0)
    }

    // fn scalar_mul(&self, scalar: f64) -> Self {
    //     let coeffs = self.coeffs.iter().map(|&c| c * scalar).collect();

    //     Polynomial::new(coeffs)
    // }

    fn shift(&self, n: usize) -> Self {
        let mut coeffs = vec![0.0; n];

        coeffs.extend_from_slice(&self.coeffs);

        Polynomial::new(coeffs)
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let max_len = self.coeffs.len().max(other.coeffs.len());
        let mut coeffs = vec![0.0; max_len];

        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }

        for (i, &c) in other.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }

        Polynomial::new(coeffs)
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let max_len = self.coeffs.len().max(other.coeffs.len());
        let mut coeffs = vec![0.0; max_len];

        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        for (i, &c) in other.coeffs.iter().enumerate() {
            coeffs[i] -= c;
        }

        Polynomial::new(coeffs)
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(mut self, mut other: Self) -> Self {
        let mut coeffs = vec![0.0; self.degree() + other.degree() + 1];

        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                coeffs[i + j] += a * b;
            }
        }

        Polynomial::new(coeffs)
    }
}

pub fn poly_long_division(a: Polynomial, mut b: Polynomial) -> (Polynomial, Polynomial) {
    let mut q = Polynomial::new(vec![0.0]);
    let mut p = a.clone();
    let d = b.degree();
    let c = b.leading_coeff();

    while p.degree() >= d {
        let lc_p = p.leading_coeff();
        let s = Polynomial::new(vec![lc_p / c]).shift(p.degree() - d);

        q = q + s.clone();
        p = p - s * b.clone();
    }

    (q, p)
}
