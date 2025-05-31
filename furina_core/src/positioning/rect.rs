use std::fmt::Display;
use std::ops::Add;

use paste::paste;
use serde::{Deserialize, Serialize};

use crate::positioning::{Pos, Scalable, Size};

#[derive(Debug, Clone, PartialEq, Default, Copy, Serialize, Deserialize)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}

impl<T> Rect<T>
where
    T: Copy,
{
    pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
        Rect { left, top, width, height }
    }

    pub fn origin(&self) -> Pos<T> {
        Pos { x: self.left, y: self.top }
    }

    pub fn size(&self) -> Size<T> {
        Size { width: self.width, height: self.height }
    }
}

impl<T> Rect<T>
where
    T: Add<T, Output = T> + Copy,
{
    pub fn translate(&self, pos: Pos<T>) -> Rect<T> {
        Rect {
            left: self.left + pos.x,
            top: self.top + pos.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl<T> Display for Rect<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect {} -> {}", self.origin(), self.size())
    }
}

impl Scalable for Rect<f64> {
    fn scale(&self, factor: f64) -> Self {
        Rect {
            left: self.left * factor,
            top: self.top * factor,
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

macro_rules! convert_rect_type {
    ($t1:ty, $t2:ty) => {
        impl Rect<$t1> {
            paste! {
                pub fn [<to_rect_ $t2>](&self) -> Rect<$t2> {
                    Rect {
                        left: self.left as $t2,
                        top: self.top as $t2,
                        width: self.width as $t2,
                        height: self.height as $t2,
                    }
                }
            }
        }
    };
}

convert_rect_type!(f64, i32);
convert_rect_type!(f64, usize);
convert_rect_type!(f64, u32);
convert_rect_type!(u32, usize);
convert_rect_type!(i32, usize);
convert_rect_type!(i32, f64);
convert_rect_type!(i32, u32);
convert_rect_type!(usize, i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_new() {
        let rect = Rect::new(10, 20, 100, 50);
        assert_eq!(rect.left, 10);
        assert_eq!(rect.top, 20);
        assert_eq!(rect.width, 100);
        assert_eq!(rect.height, 50);
    }

    #[test]
    fn test_rect_default() {
        let rect: Rect<i32> = Rect::default();
        assert_eq!(rect.left, 0);
        assert_eq!(rect.top, 0);
        assert_eq!(rect.width, 0);
        assert_eq!(rect.height, 0);
    }

    #[test]
    fn test_rect_origin() {
        let rect = Rect::new(10, 20, 100, 50);
        let origin = rect.origin();
        assert_eq!(origin.x, 10);
        assert_eq!(origin.y, 20);
    }

    #[test]
    fn test_rect_size() {
        let rect = Rect::new(10, 20, 100, 50);
        let size = rect.size();
        assert_eq!(size.width, 100);
        assert_eq!(size.height, 50);
    }

    #[test]
    fn test_rect_translate() {
        let rect = Rect::new(10, 20, 100, 50);
        let pos = Pos::new(5, 15);
        let translated = rect.translate(pos);

        assert_eq!(translated.left, 15);
        assert_eq!(translated.top, 35);
        assert_eq!(translated.width, 100); // 尺寸保持不变
        assert_eq!(translated.height, 50);
    }

    #[test]
    fn test_rect_display() {
        let rect = Rect::new(10, 20, 100, 50);
        let display_str = format!("{rect}");
        assert_eq!(display_str, "Rect (10, 20) -> Size(50, 100)");
    }

    #[test]
    fn test_rect_clone_and_equality() {
        let rect1 = Rect::new(10, 20, 100, 50);
        let rect2 = rect1;
        assert_eq!(rect1, rect2);
    }

    #[test]
    fn test_rect_scale_f64() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        let scaled = rect.scale(2.0);

        assert_eq!(scaled.left, 20.0);
        assert_eq!(scaled.top, 40.0);
        assert_eq!(scaled.width, 200.0);
        assert_eq!(scaled.height, 100.0);
    }

    #[test]
    fn test_rect_scale_fractional() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        let scaled = rect.scale(0.5);

        assert_eq!(scaled.left, 5.0);
        assert_eq!(scaled.top, 10.0);
        assert_eq!(scaled.width, 50.0);
        assert_eq!(scaled.height, 25.0);
    }

    #[test]
    fn test_rect_convert_f64_to_i32() {
        let rect = Rect::new(10.7, 20.3, 100.9, 50.1);
        let converted = rect.to_rect_i32();

        assert_eq!(converted.left, 10);
        assert_eq!(converted.top, 20);
        assert_eq!(converted.width, 100);
        assert_eq!(converted.height, 50);
    }

    #[test]
    fn test_rect_convert_f64_to_usize() {
        let rect = Rect::new(10.7, 20.3, 100.9, 50.1);
        let converted = rect.to_rect_usize();

        assert_eq!(converted.left, 10);
        assert_eq!(converted.top, 20);
        assert_eq!(converted.width, 100);
        assert_eq!(converted.height, 50);
    }

    #[test]
    fn test_rect_convert_i32_to_f64() {
        let rect = Rect::new(10_i32, 20_i32, 100_i32, 50_i32);
        let converted = rect.to_rect_f64();

        assert_eq!(converted.left, 10.0);
        assert_eq!(converted.top, 20.0);
        assert_eq!(converted.width, 100.0);
        assert_eq!(converted.height, 50.0);
    }

    #[test]
    fn test_rect_convert_u32_to_usize() {
        let rect = Rect::new(10_u32, 20_u32, 100_u32, 50_u32);
        let converted = rect.to_rect_usize();

        assert_eq!(converted.left, 10_usize);
        assert_eq!(converted.top, 20_usize);
        assert_eq!(converted.width, 100_usize);
        assert_eq!(converted.height, 50_usize);
    }

    #[test]
    fn test_rect_convert_usize_to_i32() {
        let rect = Rect::new(10_usize, 20_usize, 100_usize, 50_usize);
        let converted = rect.to_rect_i32();

        assert_eq!(converted.left, 10_i32);
        assert_eq!(converted.top, 20_i32);
        assert_eq!(converted.width, 100_i32);
        assert_eq!(converted.height, 50_i32);
    }

    #[test]
    fn test_rect_serde() {
        let rect = Rect::new(10, 20, 100, 50);
        let serialized = serde_json::to_string(&rect).unwrap();
        let deserialized: Rect<i32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(rect, deserialized);
    }

    #[test]
    fn test_rect_with_negative_coordinates() {
        let rect = Rect::new(-10, -20, 100, 50);
        let pos = Pos::new(5, 10);
        let translated = rect.translate(pos);

        assert_eq!(translated.left, -5);
        assert_eq!(translated.top, -10);
    }

    #[test]
    fn test_rect_zero_size() {
        let rect = Rect::new(10, 20, 0, 0);
        let size = rect.size();
        assert_eq!(size.width, 0);
        assert_eq!(size.height, 0);
    }
}
