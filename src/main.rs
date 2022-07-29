mod vector;
mod tween;

use std::fmt::Display;

use vector::*;
use tween::*;

fn main() {
    let p1 = vector!(1.0, 2.0, 3.0);
    let p2 = vector!(4.0, 5.0, 6.0);

    println!("P1:        {}", p1);
    println!("P2:        {}", p2);
    println!("P1 == P2:  {}", p1 == p2);
    println!("P1 + P2:   {}", p1 + p2);
    println!("P1 + 1.0:  {}", p1 + 1.0);
    println!("P1 - P2:   {}", p1 - p2);
    println!("P1 - 1.0:  {}", p1 - 1.0);
    println!("P1 * P2:   {}", p1 * p2);
    println!("P1 * 2.0:  {}", p1 * 2.0);
    println!("P1 / P2:   {}", p1 / p2);
    println!("P1 / 2.0:  {}", p1 / 2.0);

    println!("P1 . P2:   {}", Vector::dot(&p1, &p2));
    println!("P1 x P2:   {}", Vector::cross(&p1, &p2));

    let mut val: f64 = 0.0;
    let mut tween = Tween::new(&mut val, 0.0, 1.0, 1.0, &print);
    tween.start();

    val = 0.0;

    println!("test: {}", val);
}

fn print<T: Display>(value: &T) {
    println!("val: {}", value)
}
