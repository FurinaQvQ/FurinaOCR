use genshin::artifact::{ArtifactStat, ArtifactStatName};

/// 测试常见的OCR误识别属性名称能否正确解析
#[test]
fn test_ocr_misrecognition_support() {
    // 测试"暴击伤"被正确识别为CriticalDamage
    let result = ArtifactStatName::from_zh_cn("暴击伤", true);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), ArtifactStatName::CriticalDamage);

    // 测试正确的"暴击伤害"仍然被正确识别
    let result = ArtifactStatName::from_zh_cn("暴击伤害", true);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), ArtifactStatName::CriticalDamage);
}

/// 测试修复后的属性解析不再报"未知属性名称"错误
#[test]
fn test_critical_damage_parsing() {
    // 测试解析"暴击伤+22.5%"不再出现错误
    let result = ArtifactStat::from_zh_cn_raw("暴击伤+22.5%");
    assert!(result.is_some(), "应该能够成功解析'暴击伤+22.5%'");

    let stat = result.unwrap();
    assert_eq!(stat.name, ArtifactStatName::CriticalDamage);
    assert!((stat.value - 0.225).abs() < f64::EPSILON);
}

/// 测试各种暴击伤害相关的格式都能正确解析
#[test]
fn test_various_critical_damage_formats() {
    let test_cases = vec![
        ("暴击伤+22.5%", 0.225),
        ("暴击伤害+22.5%", 0.225),
        ("暴击伤+15.0%", 0.150),
        ("暴击伤害+30.2%", 0.302),
    ];

    for (input, expected_value) in test_cases {
        let result = ArtifactStat::from_zh_cn_raw(input);
        assert!(result.is_some(), "应该能够成功解析'{}'", input);

        let stat = result.unwrap();
        assert_eq!(stat.name, ArtifactStatName::CriticalDamage);
        assert!(
            (stat.value - expected_value).abs() < f64::EPSILON,
            "'{}'的解析值应该是{:.3}，实际是{:.3}",
            input,
            expected_value,
            stat.value
        );
    }
}
