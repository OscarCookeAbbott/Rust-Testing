mod vector;
use vector::*;

fn main() -> () {
    let p1 = Vector::new(1.0, 2.0, 3.0);
    let p2 = Vector::new(4.0, 5.0, 6.0);

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
}
