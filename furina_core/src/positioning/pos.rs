use std::fmt::Display;
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

use crate::positioning::{Scalable, Size};

#[derive(Debug, Clone, PartialEq, Default, Copy, Serialize, Deserialize)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add<Pos<T>> for Pos<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Pos<T>) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> Add<Size<T>> for Pos<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Size<T>) -> Self::Output {
        Pos { x: self.x + rhs.width, y: self.y + rhs.height }
    }
}

impl<T> Sub<Pos<T>> for Pos<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Pos<T>) -> Self::Output {
        Pos { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T> Pos<T> {
    pub fn new(x: T, y: T) -> Pos<T> {
        Pos { x, y }
    }
}

impl<T> Display for Pos<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Scalable for Pos<f64> {
    fn scale(&self, factor: f64) -> Pos<f64> {
        Pos { x: self.x * factor, y: self.y * factor }
    }
}

macro_rules! impl_int_pos {
    ($t:ty) => {
        impl Scalable for Pos<$t> {
            fn scale(&self, factor: f64) -> Pos<$t> {
                Pos { x: ((self.x as f64) * factor) as $t, y: ((self.y as f64) * factor) as $t }
            }
        }
    };
}

impl_int_pos!(i32);
impl_int_pos!(usize);
impl_int_pos!(u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_new() {
        let pos = Pos::new(10, 20);
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 20);
    }

    #[test]
    fn test_pos_default() {
        let pos: Pos<i32> = Pos::default();
        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_pos_add() {
        let pos1 = Pos::new(10, 20);
        let pos2 = Pos::new(5, 15);
        let result = pos1 + pos2;
        assert_eq!(result.x, 15);
        assert_eq!(result.y, 35);
    }

    #[test]
    fn test_pos_add_size() {
        let pos = Pos::new(10, 20);
        let size = Size::new(100, 50);
        let result = pos + size;
        assert_eq!(result.x, 110);
        assert_eq!(result.y, 70);
    }

    #[test]
    fn test_pos_sub() {
        let pos1 = Pos::new(10, 20);
        let pos2 = Pos::new(5, 15);
        let result = pos1 - pos2;
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 5);
    }

    #[test]
    fn test_pos_display() {
        let pos = Pos::new(10, 20);
        assert_eq!(format!("{pos}"), "(10, 20)");
    }

    #[test]
    fn test_pos_clone_and_equality() {
        let pos1 = Pos::new(10, 20);
        let pos2 = pos1;
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_pos_scale_f64() {
        let pos = Pos::new(10.0, 20.0);
        let scaled = pos.scale(2.0);
        assert_eq!(scaled.x, 20.0);
        assert_eq!(scaled.y, 40.0);
    }

    #[test]
    fn test_pos_scale_i32() {
        let pos = Pos::new(10_i32, 20_i32);
        let scaled = pos.scale(2.5);
        assert_eq!(scaled.x, 25);
        assert_eq!(scaled.y, 50);
    }

    #[test]
    fn test_pos_scale_usize() {
        let pos = Pos::new(10_usize, 20_usize);
        let scaled = pos.scale(1.5);
        assert_eq!(scaled.x, 15);
        assert_eq!(scaled.y, 30);
    }

    #[test]
    fn test_pos_scale_u32() {
        let pos = Pos::new(10_u32, 20_u32);
        let scaled = pos.scale(0.5);
        assert_eq!(scaled.x, 5);
        assert_eq!(scaled.y, 10);
    }

    #[test]
    fn test_pos_with_negative_values() {
        let pos = Pos::new(-10, -20);
        let pos2 = Pos::new(5, 10);
        let result = pos + pos2;
        assert_eq!(result.x, -5);
        assert_eq!(result.y, -10);
    }

    #[test]
    fn test_pos_serde() {
        let pos = Pos::new(10, 20);
        let serialized = serde_json::to_string(&pos).unwrap();
        let deserialized: Pos<i32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pos, deserialized);
    }
}
