use super::*;

#[test]
fn add() {
    let v1 = vector!(-1.0, 2.0, -3.0);
    let v2 = vector!(6.0, -5.0, 4.0);
    let value = vector!(5.0, -3.0, 1.0);
    assert_eq!(v1 + v2, value)
}

#[test]
fn add_assign() {
    let mut v1 = vector!(-1.0, 2.0, -3.0);
    v1 += vector!(6.0, -5.0, 4.0);
    let value = vector!(5.0, -3.0, 1.0);
    assert_eq!(v1, value)
}

#[test]
fn sub() {
    let v1 = vector!(-1.0, 2.0, -3.0);
    let v2 = vector!(6.0, -5.0, 4.0);
    let value = vector!(-7.0, 7.0, -7.0);
    assert_eq!(v1 - v2, value)
}

#[test]
fn sub_assign() {
    let mut v1 = vector!(-1.0, 2.0, -3.0);
    v1 -= vector!(6.0, -5.0, 4.0);
    let value = vector!(-7.0, 7.0, -7.0);
    assert_eq!(v1, value)
}

#[test]
fn mul() {
    let v1 = vector!(-1.0, 2.0, -3.0);
    let v2 = vector!(6.0, -5.0, 4.0);
    let value = vector!(-6.0, -10.0, -12.0);
    assert_eq!(v1 * v2, value)
}

#[test]
fn mul_assign() {
    let mut v1 = vector!(-1.0, 2.0, -3.0);
    v1 *= vector!(6.0, -5.0, 4.0);
    let value = vector!(-6.0, -10.0, -12.0);
    assert_eq!(v1, value)
}

#[test]
fn div() {
    let v1 = vector!(-1.0, 2.0, -3.0);
    let v2 = vector!(6.0, -5.0, 4.0);
    let value = vector!(-1.0 / 6.0, -2.0 / 5.0, -3.0 / 4.0);
    assert_eq!(v1 / v2, value)
}

#[test]
fn div_assign() {
    let mut v1 = vector!(-1.0, 2.0, -3.0);
    v1 /= vector!(6.0, -5.0, 4.0);
    let value = vector!(-1.0 / 6.0, -2.0 / 5.0, -3.0 / 4.0);
    assert_eq!(v1, value)
}

#[test]
fn neg() {
    let v1 = vector!(-1.0, 2.0, -3.0);
    let value = vector!(1.0, -2.0, 3.0);
    assert_eq!(-v1, value)
}

#[test]
fn dot() {
    let v1 = vector!(-1.0, 2.0, -3.0);
    let v2 = vector!(6.0, -5.0, 4.0);
    let value = -28.0;
    assert_eq!(Vector::dot(&v1, &v2), value)
}

#[test]
fn cross() {
    let v1 = vector!(-1.0, 2.0, -3.0);
    let v2 = vector!(6.0, -5.0, 4.0);
    let value = vector!(-7.0, -14.0, -7.0);
    assert_eq!(Vector::cross(&v1, &v2), value)
}
