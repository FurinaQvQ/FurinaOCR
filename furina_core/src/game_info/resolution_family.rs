use serde::{Deserialize, Serialize};

use crate::positioning::Size;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResolutionFamily {
    // 仅支持以下分辨率族
    Windows16x9, // 2560×1440, 1920×1080, 1600×900
}

impl ResolutionFamily {
    pub fn new(width: u32, height: u32) -> Result<ResolutionFamily, anyhow::Error> {
        match (width, height) {
            // 支持的3种分辨率
            (2560, 1440) => Ok(ResolutionFamily::Windows16x9),
            (1920, 1080) => Ok(ResolutionFamily::Windows16x9),
            (1600, 900) => Ok(ResolutionFamily::Windows16x9),

            // 不支持的分辨率
            _ => Err(anyhow::anyhow!(
                "不支持的分辨率: {}×{}\n支持的分辨率:\n- 2560×1440\n- 1920×1080\n- 1600×900",
                width,
                height
            )),
        }
    }
}
