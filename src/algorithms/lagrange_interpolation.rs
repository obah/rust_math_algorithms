use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

fn _lagrange_interpolation<T>(points: &[Point<T>]) -> impl Fn(T) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + From<f64>,
{
    let l_j = |j: usize, x: T| {
        let mut product = T::from(1.0);
        let x_j = points[j].x;

        for (i, point) in points.iter().enumerate() {
            if i != j {
                product = product * (x - point.x) / (x_j - point.x);
            }
        }
        product
    };

    move |x: T| {
        points
            .iter()
            .enumerate()
            .map(|(j, point)| point.y * l_j(j, x))
            .fold(T::from(0.0), |acc, term| acc + term)
    }
}

pub fn lagrange_interpolation() {
    let points = [
        Point { x: 1.0, y: 2.0 },
        Point { x: 2.0, y: 3.0 },
        Point { x: 4.0, y: 5.0 },
    ];

    let polynomial = _lagrange_interpolation(&points);

    for x in [1.0, 2.0, 3.0, 4.0].iter() {
        println!("P({}) = {}", x, polynomial(*x));
    }
}
