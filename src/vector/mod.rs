use std::cmp::*;
use std::fmt::*;
use std::ops::*;

/// A basic struct which covers the fundamentals of vector math.
#[derive(Default, Debug, Copy)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T> {
    /// Creates a new `2D Vector` with the given coordinates.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Neg for Vector<T> where T: Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Vector<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    /// Computes the dot product of two vectors.
    pub fn dot(a: &Self, b: &Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

impl<T> Vector<T>
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Neg<Output = T>,
{
    pub fn cross(a: &Self, b: &Self) -> Self {
        Self::new(a.y * b.z - a.z * b.y, -(a.x * b.z - a.z * b.x), a.x * b.y - a.y * b.x)
    }
}

impl<T: Display> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y)
    }
}

impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone(), z: self.z.clone() }
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<T> for Vector<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T> AddAssign for Vector<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> AddAssign<T> for Vector<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Sub<T> for Vector<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T> SubAssign for Vector<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> SubAssign<T> for Vector<T>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<T> Mul for Vector<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> MulAssign for Vector<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> Div for Vector<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T> DivAssign for Vector<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T> DivAssign<T> for Vector<T>
where
    T: Copy + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn add() {
        let v1 = Vector::new(-1.0, 2.0, -3.0);
        let v2 = Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(5.0, -3.0, 1.0);
        assert_eq!(v1 + v2, value)
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vector::new(-1.0, 2.0, -3.0);
        v1 += Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(5.0, -3.0, 1.0);
        assert_eq!(v1, value)
    }

    #[test]
    fn sub() {
        let v1 = Vector::new(-1.0, 2.0, -3.0);
        let v2 = Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(-7.0, 7.0, -7.0);
        assert_eq!(v1 - v2, value)
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vector::new(-1.0, 2.0, -3.0);
        v1 -= Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(-7.0, 7.0, -7.0);
        assert_eq!(v1, value)
    }

    #[test]
    fn mul() {
        let v1 = Vector::new(-1.0, 2.0, -3.0);
        let v2 = Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(-6.0, -10.0, -12.0);
        assert_eq!(v1 * v2, value)
    }

    #[test]
    fn mul_assign() {
        let mut v1 = Vector::new(-1.0, 2.0, -3.0);
        v1 *= Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(-6.0, -10.0, -12.0);
        assert_eq!(v1, value)
    }

    #[test]
    fn div() {
        let v1 = Vector::new(-1.0, 2.0, -3.0);
        let v2 = Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(-1.0 / 6.0, -2.0 / 5.0, -3.0 / 4.0);
        assert_eq!(v1 / v2, value)
    }

    #[test]
    fn div_assign() {
        let mut v1 = Vector::new(-1.0, 2.0, -3.0);
        v1 /= Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(-1.0 / 6.0, -2.0 / 5.0, -3.0 / 4.0);
        assert_eq!(v1, value)
    }

    #[test]
    fn neg() {
        let v1 = Vector::new(-1.0, 2.0, -3.0);
        let value = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-v1, value)
    }

    #[test]
    fn dot() {
        let v1 = Vector::new(-1.0, 2.0, -3.0);
        let v2 = Vector::new(6.0, -5.0, 4.0);
        let value = -28.0;
        assert_eq!(Vector::dot(&v1, &v2), value)
    }

    #[test]
    fn cross() {
        let v1 = Vector::new(-1.0, 2.0, -3.0);
        let v2 = Vector::new(6.0, -5.0, 4.0);
        let value = Vector::new(-7.0, -14.0, -7.0);
        assert_eq!(Vector::cross(&v1, &v2), value)
    }
}
