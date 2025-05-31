use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::positioning::Scalable;

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy, Serialize, Deserialize)]
pub struct Size<T> {
    pub height: T,
    pub width: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Size<T> {
        Size { width, height }
    }
}

impl<T> Display for Size<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size({}, {})", self.height, self.width)
    }
}

macro_rules! impl_int_size {
    ($t:ty) => {
        impl Scalable for Size<$t> {
            fn scale(&self, factor: f64) -> Self {
                Size {
                    height: ((self.height as f64) * factor) as $t,
                    width: ((self.width as f64) * factor) as $t,
                }
            }
        }
    };
}

impl Scalable for Size<f64> {
    fn scale(&self, factor: f64) -> Self {
        Size { height: self.height * factor, width: self.width * factor }
    }
}

impl_int_size!(i32);
impl_int_size!(usize);
impl_int_size!(u32);

macro_rules! impl_int_hash {
    ($t:ty) => {
        impl Hash for Size<$t> {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.width.hash(state);
                self.height.hash(state);
            }
        }
    };
}

impl_int_hash!(i32);
impl_int_hash!(usize);
impl_int_hash!(u32);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_size_new() {
        let size = Size::new(100, 50);
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 50);
    }

    #[test]
    fn test_size_default() {
        let size: Size<i32> = Size::default();
        assert_eq!(size.width, 0);
        assert_eq!(size.height, 0);
    }

    #[test]
    fn test_size_display() {
        let size = Size::new(100, 50);
        assert_eq!(format!("{size}"), "Size(50, 100)"); // height, width 顺序
    }

    #[test]
    fn test_size_clone_and_equality() {
        let size1 = Size::new(100, 50);
        let size2 = size1;
        assert_eq!(size1, size2);
    }

    #[test]
    fn test_size_scale_f64() {
        let size = Size::new(100.0, 50.0);
        let scaled = size.scale(2.0);
        assert_eq!(scaled.width, 200.0);
        assert_eq!(scaled.height, 100.0);
    }

    #[test]
    fn test_size_scale_i32() {
        let size = Size::new(100_i32, 50_i32);
        let scaled = size.scale(1.5);
        assert_eq!(scaled.width, 150);
        assert_eq!(scaled.height, 75);
    }

    #[test]
    fn test_size_scale_usize() {
        let size = Size::new(100_usize, 50_usize);
        let scaled = size.scale(0.5);
        assert_eq!(scaled.width, 50);
        assert_eq!(scaled.height, 25);
    }

    #[test]
    fn test_size_scale_u32() {
        let size = Size::new(100_u32, 50_u32);
        let scaled = size.scale(2.5);
        assert_eq!(scaled.width, 250);
        assert_eq!(scaled.height, 125);
    }

    #[test]
    fn test_size_hash_i32() {
        let size1 = Size::new(100_i32, 50_i32);
        let size2 = Size::new(100_i32, 50_i32);
        let size3 = Size::new(200_i32, 50_i32);

        let mut map = HashMap::new();
        map.insert(size1, "first");

        assert!(map.contains_key(&size2)); // 相同的size应该有相同的hash
        assert!(!map.contains_key(&size3)); // 不同的size应该有不同的hash
    }

    #[test]
    fn test_size_hash_usize() {
        let size1 = Size::new(100_usize, 50_usize);
        let size2 = Size::new(100_usize, 50_usize);

        let mut map = HashMap::new();
        map.insert(size1, "test");
        assert!(map.contains_key(&size2));
    }

    #[test]
    fn test_size_hash_u32() {
        let size1 = Size::new(100_u32, 50_u32);
        let size2 = Size::new(100_u32, 50_u32);

        let mut map = HashMap::new();
        map.insert(size1, "test");
        assert!(map.contains_key(&size2));
    }

    #[test]
    fn test_size_serde() {
        let size = Size::new(100, 50);
        let serialized = serde_json::to_string(&size).unwrap();
        let deserialized: Size<i32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(size, deserialized);
    }

    #[test]
    fn test_size_zero_values() {
        let size = Size::new(0, 0);
        assert_eq!(size.width, 0);
        assert_eq!(size.height, 0);

        let scaled = size.scale(5.0);
        assert_eq!(scaled.width, 0);
        assert_eq!(scaled.height, 0);
    }

    #[test]
    fn test_size_large_values() {
        let size = Size::new(u32::MAX, u32::MAX);
        assert_eq!(size.width, u32::MAX);
        assert_eq!(size.height, u32::MAX);
    }
}
