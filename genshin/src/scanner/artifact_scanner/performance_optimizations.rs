use anyhow::Result;
use furina_core::ocr::{ImageToText, OcrModel};
use furina_core::ocr_model;
use furina_core::positioning::Rect;
use image::{ImageBuffer, Luma, RgbImage};

/// 性能优化模块
///
/// 包含各种性能优化功能：
/// - 图像处理缓存
/// - 批量处理优化
/// - 内存池管理

/// 创建新的OCR模型实例（线程安全版本）
pub fn create_ocr_model() -> Result<Box<dyn ImageToText<RgbImage> + Send>> {
    let model: Box<dyn ImageToText<RgbImage> + Send> = Box::new(
        ocr_model!("./models/model_training.onnx", "./models/index_2_word.json")
            .map_err(|e| anyhow::anyhow!("Failed to load OCR model: {}", e))?,
    );
    Ok(model)
}

/// 图像缓存池，避免频繁内存分配
#[allow(dead_code)]
struct ImagePool {
    rgb_images: Vec<RgbImage>,
    gray_images: Vec<ImageBuffer<Luma<f32>, Vec<f32>>>,
}

#[allow(dead_code)]
impl ImagePool {
    fn new() -> Self {
        Self { rgb_images: Vec::with_capacity(16), gray_images: Vec::with_capacity(16) }
    }

    /// 获取一个RGB图像，复用现有的或创建新的
    fn get_rgb_image(&mut self, width: u32, height: u32) -> RgbImage {
        if let Some(mut img) = self.rgb_images.pop() {
            if img.width() == width && img.height() == height {
                // 重置图像内容但保持尺寸
                for pixel in img.pixels_mut() {
                    *pixel = image::Rgb([0, 0, 0]);
                }
                return img;
            }
        }
        RgbImage::new(width, height)
    }

    /// 归还RGB图像到池中
    fn return_rgb_image(&mut self, img: RgbImage) {
        if self.rgb_images.len() < 16 {
            self.rgb_images.push(img);
        }
    }

    /// 获取灰度图像
    fn get_gray_image(&mut self, width: u32, height: u32) -> ImageBuffer<Luma<f32>, Vec<f32>> {
        if let Some(mut img) = self.gray_images.pop() {
            if img.width() == width && img.height() == height {
                for pixel in img.pixels_mut() {
                    pixel.0[0] = 0.0;
                }
                return img;
            }
        }
        ImageBuffer::new(width, height)
    }

    /// 归还灰度图像到池中
    fn return_gray_image(&mut self, img: ImageBuffer<Luma<f32>, Vec<f32>>) {
        if self.gray_images.len() < 16 {
            self.gray_images.push(img);
        }
    }
}

thread_local! {
    static IMAGE_POOL: std::cell::RefCell<ImagePool> = std::cell::RefCell::new(ImagePool::new());
}

/// 性能优化的OCR识别器
pub struct OptimizedOCRRecognizer {
    model: Box<dyn ImageToText<RgbImage> + Send>,
}

impl OptimizedOCRRecognizer {
    /// 创建新的优化OCR识别器
    pub fn new() -> Result<Self> {
        Ok(Self { model: create_ocr_model()? })
    }

    /// 批量OCR识别，提高处理效率
    pub fn batch_recognize(&self, images: &[RgbImage]) -> Vec<Result<String>> {
        images.iter().map(|img| self.model.image_to_text(img, false)).collect()
    }

    /// 单次OCR识别
    pub fn recognize(&self, image: &RgbImage) -> Result<String> {
        self.model.image_to_text(image, false)
    }
}

/// 优化的图像处理函数
pub struct OptimizedImageProcessor;

impl OptimizedImageProcessor {
    /// 优化的图像裁剪，减少内存分配
    pub fn crop_optimized(image: &RgbImage, rect: &Rect<f64>) -> RgbImage {
        let x = rect.left.max(0.0) as u32;
        let y = rect.top.max(0.0) as u32;
        let width = rect.width.min(image.width() as f64 - x as f64) as u32;
        let height = rect.height.min(image.height() as f64 - y as f64) as u32;

        IMAGE_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            let mut cropped = pool.get_rgb_image(width, height);

            // 直接拷贝像素数据，避免使用view().to_image()
            for dy in 0..height {
                for dx in 0..width {
                    if x + dx < image.width() && y + dy < image.height() {
                        let pixel = image.get_pixel(x + dx, y + dy);
                        cropped.put_pixel(dx, dy, *pixel);
                    }
                }
            }

            cropped
        })
    }

    /// 优化的颜色距离计算，使用内联优化
    #[inline(always)]
    pub fn color_distance_fast(c1: &image::Rgb<u8>, c2: &image::Rgb<u8>) -> u32 {
        let dr = c1.0[0] as i32 - c2.0[0] as i32;
        let dg = c1.0[1] as i32 - c2.0[1] as i32;
        let db = c1.0[2] as i32 - c2.0[2] as i32;

        // 使用平方计算，避免sqrt调用
        (dr * dr + dg * dg + db * db) as u32
    }

    /// 批量颜色距离计算
    pub fn batch_color_distance(colors: &[image::Rgb<u8>], target: &image::Rgb<u8>) -> Vec<u32> {
        colors.iter().map(|c| Self::color_distance_fast(c, target)).collect()
    }
}

/// 性能监控器
pub struct PerformanceMonitor {
    start_time: std::time::Instant,
    ocr_times: Vec<std::time::Duration>,
    capture_times: Vec<std::time::Duration>,
}

#[allow(dead_code)]
impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            ocr_times: Vec::new(),
            capture_times: Vec::new(),
        }
    }

    pub fn record_ocr_time(&mut self, duration: std::time::Duration) {
        self.ocr_times.push(duration);
    }

    pub fn record_capture_time(&mut self, duration: std::time::Duration) {
        self.capture_times.push(duration);
    }

    pub fn get_performance_summary(&self) -> String {
        let total_time = self.start_time.elapsed();
        let avg_ocr_time = if !self.ocr_times.is_empty() {
            self.ocr_times.iter().sum::<std::time::Duration>() / self.ocr_times.len() as u32
        } else {
            std::time::Duration::from_millis(0)
        };
        let avg_capture_time = if !self.capture_times.is_empty() {
            self.capture_times.iter().sum::<std::time::Duration>() / self.capture_times.len() as u32
        } else {
            std::time::Duration::from_millis(0)
        };

        format!(
            "性能统计 - 总时间: {:?}, 平均OCR时间: {:?}, 平均捕获时间: {:?}, OCR次数: {}, 捕获次数: {}",
            total_time, avg_ocr_time, avg_capture_time, self.ocr_times.len(), self.capture_times.len()
        )
    }
}

/// 自适应延时管理器
pub struct AdaptiveDelayManager {
    base_delay: u32,
    current_delay: u32,
    success_count: u32,
    failure_count: u32,
    last_adjustment: std::time::Instant,
}

impl AdaptiveDelayManager {
    pub fn new(base_delay: u32) -> Self {
        Self {
            base_delay,
            current_delay: base_delay,
            success_count: 0,
            failure_count: 0,
            last_adjustment: std::time::Instant::now(),
        }
    }

    /// 记录成功操作
    pub fn record_success(&mut self) {
        self.success_count += 1;
        self.adjust_delay();
    }

    /// 记录失败操作
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.adjust_delay();
    }

    /// 获取当前建议的延时
    pub fn get_current_delay(&self) -> u32 {
        self.current_delay
    }

    /// 自动调整延时
    fn adjust_delay(&mut self) {
        // 每5秒调整一次
        if self.last_adjustment.elapsed().as_secs() < 5 {
            return;
        }

        let total_operations = self.success_count + self.failure_count;
        if total_operations == 0 {
            return;
        }

        let success_rate = self.success_count as f64 / total_operations as f64;

        if success_rate > 0.95 {
            // 成功率很高，可以减少延时
            self.current_delay =
                (self.current_delay as f64 * 0.9).max(self.base_delay as f64 / 2.0) as u32;
        } else if success_rate < 0.8 {
            // 成功率较低，增加延时
            self.current_delay =
                (self.current_delay as f64 * 1.2).min(self.base_delay as f64 * 2.0) as u32;
        }

        // 重置计数器
        self.success_count = 0;
        self.failure_count = 0;
        self.last_adjustment = std::time::Instant::now();
    }
}
