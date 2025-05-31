// 公共模块声明
mod capturer;
mod generic_capturer;
mod stream_capturer;

// Windows平台特定模块
#[cfg(target_os = "windows")]
mod screenshots_capturer;
#[cfg(target_os = "windows")]
mod winapi_capturer;
#[cfg(target_os = "windows")]
mod windows_capturer;

// Linux平台特定模块
#[cfg(target_os = "linux")]
mod libwayshot_capturer;

// 公共导出
pub use capturer::Capturer;
pub use generic_capturer::GenericCapturer;
// Linux平台导出
#[cfg(target_os = "linux")]
pub use libwayshot_capturer::LibwayshotCapturer;
// Windows平台导出
#[cfg(target_os = "windows")]
pub use screenshots_capturer::ScreenshotsCapturer;
pub use stream_capturer::StreamingCapturer;
#[cfg(target_os = "windows")]
pub use winapi_capturer::WinapiCapturer;
#[cfg(target_os = "windows")]
pub use windows_capturer::WindowsCapturer;

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use image::RgbImage;

    use super::*;
    use crate::positioning::{Pos, Rect};

    /// Mock屏幕捕获器，用于测试
    struct MockCapturer {
        pub capture_count: Arc<Mutex<usize>>,
        pub should_fail: bool,
        pub test_image: RgbImage,
    }

    impl MockCapturer {
        fn new(width: u32, height: u32) -> Self {
            let mut test_image = RgbImage::new(width, height);

            // 创建一个简单的测试图案
            for x in 0..width {
                for y in 0..height {
                    let red = ((x * 255) / width) as u8;
                    let green = ((y * 255) / height) as u8;
                    let blue = 128;
                    test_image.put_pixel(x, y, image::Rgb([red, green, blue]));
                }
            }

            Self { capture_count: Arc::new(Mutex::new(0)), should_fail: false, test_image }
        }

        fn new_failing() -> Self {
            Self {
                capture_count: Arc::new(Mutex::new(0)),
                should_fail: true,
                test_image: RgbImage::new(1, 1),
            }
        }
    }

    impl Capturer<RgbImage> for MockCapturer {
        fn capture_rect(&self, rect: Rect<i32>) -> anyhow::Result<RgbImage> {
            let mut count = self.capture_count.lock().unwrap();
            *count += 1;

            if self.should_fail {
                anyhow::bail!("模拟捕获失败");
            }

            // 模拟区域裁剪
            let width = rect.width.min(self.test_image.width() as i32).max(1) as u32;
            let height = rect.height.min(self.test_image.height() as i32).max(1) as u32;

            let mut cropped = RgbImage::new(width, height);
            for x in 0..width {
                for y in 0..height {
                    let src_x = (rect.left + x as i32).max(0) as u32;
                    let src_y = (rect.top + y as i32).max(0) as u32;

                    if src_x < self.test_image.width() && src_y < self.test_image.height() {
                        let pixel = self.test_image.get_pixel(src_x, src_y);
                        cropped.put_pixel(x, y, *pixel);
                    }
                }
            }

            Ok(cropped)
        }

        fn capture_relative_to(
            &self,
            rect: Rect<i32>,
            _base_pos: Pos<i32>,
        ) -> anyhow::Result<RgbImage> {
            // 调用capture_rect来避免重复代码
            self.capture_rect(rect)
        }
    }

    impl MockCapturer {
        fn capture(&self) -> anyhow::Result<RgbImage> {
            let full_rect =
                Rect::new(0, 0, self.test_image.width() as i32, self.test_image.height() as i32);
            self.capture_rect(full_rect)
        }
    }

    #[test]
    fn test_capturer_trait_basic_functionality() {
        let capturer = MockCapturer::new(100, 80);

        // 测试基本捕获功能
        let result = capturer.capture();
        assert!(result.is_ok());

        let image = result.unwrap();
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 80);

        // 验证捕获计数
        let count = *capturer.capture_count.lock().unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_capturer_relative_capture() {
        let capturer = MockCapturer::new(200, 150);
        let rect = Rect::new(10, 20, 50, 40);
        let base_pos = Pos::new(0, 0);

        let result = capturer.capture_relative_to(rect, base_pos);
        assert!(result.is_ok());

        let image = result.unwrap();
        assert_eq!(image.width(), 50);
        assert_eq!(image.height(), 40);

        let count = *capturer.capture_count.lock().unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_capturer_error_handling() {
        let capturer = MockCapturer::new_failing();

        // 测试基本捕获错误
        let result = capturer.capture();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("模拟捕获失败"));

        // 测试相对捕获错误
        let rect = Rect::new(0, 0, 10, 10);
        let base_pos = Pos::new(0, 0);
        let result = capturer.capture_relative_to(rect, base_pos);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("模拟捕获失败"));
    }

    #[test]
    fn test_capturer_boundary_conditions() {
        let capturer = MockCapturer::new(100, 100);

        // 测试超出边界的矩形
        let oversized_rect = Rect::new(50, 50, 200, 200);
        let base_pos = Pos::new(0, 0);

        let result = capturer.capture_relative_to(oversized_rect, base_pos);
        assert!(result.is_ok());

        let image = result.unwrap();
        // 应该被裁剪到图像边界内
        assert!(image.width() <= 100);
        assert!(image.height() <= 100);
    }

    #[test]
    fn test_capturer_negative_coordinates() {
        let capturer = MockCapturer::new(100, 100);

        // 测试负坐标
        let rect = Rect::new(-10, -10, 50, 50);
        let base_pos = Pos::new(0, 0);

        let result = capturer.capture_relative_to(rect, base_pos);
        assert!(result.is_ok());

        let image = result.unwrap();
        assert!(image.width() > 0);
        assert!(image.height() > 0);
    }

    #[test]
    fn test_capturer_zero_size_rect() {
        let capturer = MockCapturer::new(100, 100);

        // 测试零尺寸矩形
        let rect = Rect::new(10, 10, 0, 0);
        let base_pos = Pos::new(0, 0);

        let result = capturer.capture_relative_to(rect, base_pos);
        assert!(result.is_ok());

        let image = result.unwrap();
        // 最小应该是1x1
        assert_eq!(image.width(), 1);
        assert_eq!(image.height(), 1);
    }

    #[test]
    fn test_capturer_multiple_captures() {
        let capturer = MockCapturer::new(50, 50);

        // 测试多次捕获
        for i in 1..=5 {
            let result = capturer.capture();
            assert!(result.is_ok());

            let count = *capturer.capture_count.lock().unwrap();
            assert_eq!(count, i);
        }
    }

    #[test]
    fn test_capturer_pixel_accuracy() {
        let capturer = MockCapturer::new(10, 10);

        let image = capturer.capture().unwrap();

        // 验证像素值是否符合我们的测试图案
        let pixel_00 = image.get_pixel(0, 0);
        let pixel_99 = image.get_pixel(9, 9);

        // 左上角应该是红色较少，绿色较少
        assert_eq!(pixel_00.0[0], 0); // red = (0 * 255) / 10 = 0
        assert_eq!(pixel_00.0[1], 0); // green = (0 * 255) / 10 = 0
        assert_eq!(pixel_00.0[2], 128); // blue = 128

        // 右下角应该有更多红色和绿色
        assert!(pixel_99.0[0] > pixel_00.0[0]);
        assert!(pixel_99.0[1] > pixel_00.0[1]);
        assert_eq!(pixel_99.0[2], 128); // blue保持不变
    }

    #[test]
    fn test_capturer_thread_safety() {
        use std::thread;

        let capturer = Arc::new(MockCapturer::new(100, 100));
        let mut handles = vec![];

        // 启动多个线程同时进行捕获
        for _ in 0..5 {
            let capturer_clone = Arc::clone(&capturer);
            let handle = thread::spawn(move || {
                let result = capturer_clone.capture();
                assert!(result.is_ok());
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        // 验证所有捕获都被计数
        let count = *capturer.capture_count.lock().unwrap();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_capturer_performance_simulation() {
        use std::time::{Duration, Instant};

        let capturer = MockCapturer::new(1920, 1080); // 模拟高分辨率
        let start = Instant::now();

        // 模拟连续捕获
        for _ in 0..10 {
            let result = capturer.capture();
            assert!(result.is_ok());
        }

        let elapsed = start.elapsed();

        // 验证性能指标（调试模式下给予更宽松的时间限制）
        assert!(elapsed < Duration::from_millis(5000), "性能测试超时: {elapsed:?}");

        let count = *capturer.capture_count.lock().unwrap();
        assert_eq!(count, 10);
    }
}
