use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Default, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Point<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<Point<f64>> for Point<i32> {
    fn from(v: Point<f64>) -> Self {
        Point {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}

impl From<Point<i32>> for Point<f64> {
    fn from(v: Point<i32>) -> Self {
        Point {
            x: v.x as f64,
            y: v.y as f64,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl From<Vec3<f64>> for Vec3<i32> {
    fn from(v: Vec3<f64>) -> Self {
        Vec3 {
            x: v.x as i32,
            y: v.y as i32,
            z: v.z as i32,
        }
    }
}

impl From<Vec3<i32>> for Vec3<f64> {
    fn from(v: Vec3<i32>) -> Self {
        Vec3 {
            x: v.x as f64,
            y: v.y as f64,
            z: v.z as f64,
        }
    }
}
