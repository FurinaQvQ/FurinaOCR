fn get_hoarfrost_offset(window_width: u32, window_height: u32) -> i32 {
    match (window_width, window_height) {
        // 仅支持3种特定分辨率的偏移量（基于严格测量的正确值）
        (2560, 1440) => 51,  // 严格测量确认
        (1920, 1080) => 35,  // 调整后避免副词条重叠
        (1600, 900) => 32,   // 严格测量确认
        // 其他所有分辨率不支持，返回0
        _ => {
            log::warn!("不支持的分辨率 {}x{}，祝圣之霜偏移量设为0", window_width, window_height);
            0
        }
    }
} 