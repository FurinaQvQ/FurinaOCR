pub trait Scalable {
    fn scale(&self, factor: f64) -> Self;
}

impl Scalable for f64 {
    fn scale(&self, factor: f64) -> Self {
        *self * factor
    }
}

macro_rules! impl_int_scale {
    ($t:ty) => {
        impl Scalable for $t {
            fn scale(&self, factor: f64) -> Self {
                ((*self as f64) * factor) as $t
            }
        }
    };
}

impl_int_scale!(i32);
impl_int_scale!(usize);
impl_int_scale!(u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_scaling() {
        let value = 10.5;
        assert_eq!(value.scale(2.0), 21.0);
        assert_eq!(value.scale(0.5), 5.25);
        assert_eq!(value.scale(1.0), 10.5);
    }

    #[test]
    fn test_i32_scaling() {
        let value = 10_i32;
        assert_eq!(value.scale(2.0), 20);
        assert_eq!(value.scale(0.5), 5);
        assert_eq!(value.scale(1.5), 15);
        assert_eq!(value.scale(1.0), 10);
    }

    #[test]
    fn test_usize_scaling() {
        let value = 10_usize;
        assert_eq!(value.scale(2.0), 20);
        assert_eq!(value.scale(0.5), 5);
        assert_eq!(value.scale(1.5), 15);
        assert_eq!(value.scale(1.0), 10);
    }

    #[test]
    fn test_u32_scaling() {
        let value = 10_u32;
        assert_eq!(value.scale(2.0), 20);
        assert_eq!(value.scale(0.5), 5);
        assert_eq!(value.scale(1.5), 15);
        assert_eq!(value.scale(1.0), 10);
    }

    #[test]
    fn test_zero_scaling() {
        assert_eq!(0_i32.scale(5.0), 0);
        assert_eq!(0_usize.scale(5.0), 0);
        assert_eq!(0_u32.scale(5.0), 0);
        assert_eq!(0.0_f64.scale(5.0), 0.0);
    }

    #[test]
    fn test_negative_scaling() {
        let value = 10_i32;
        assert_eq!(value.scale(-1.0), -10);
        assert_eq!(value.scale(-0.5), -5);

        let float_value = 10.5_f64;
        assert_eq!(float_value.scale(-2.0), -21.0);
    }

    #[test]
    fn test_fractional_scaling() {
        let value = 7_i32;
        assert_eq!(value.scale(1.4), 9); // 7 * 1.4 = 9.8 -> 9
        assert_eq!(value.scale(1.6), 11); // 7 * 1.6 = 11.2 -> 11

        let float_value = 7.0_f64;
        assert!((float_value.scale(1.4) - 9.8).abs() < 0.001);
        assert!((float_value.scale(1.6) - 11.2).abs() < 0.001);
    }
}
