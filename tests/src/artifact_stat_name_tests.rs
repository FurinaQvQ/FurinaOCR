use genshin::artifact::{ArtifactStat, ArtifactStatName};

/// Test common OCR misrecognition artifact names can be parsed correctly
#[test]
fn test_artifact_stat_name_resolution() {
    // 测试"暴击伤"被正确识别为CriticalDamage
    let result = ArtifactStatName::from_zh_cn("暴击伤", true);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), ArtifactStatName::CriticalDamage);

    // 测试正确的"暴击伤害"仍然被正确识别
    let result = ArtifactStatName::from_zh_cn("暴击伤害", true);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), ArtifactStatName::CriticalDamage);
}

/// Test fixed attribute parsing no longer reports "unknown attribute name" errors
#[test]
fn test_attribute_parsing_fixes() {
    // 测试解析"暴击伤+22.5%"不再出现错误
    let result = ArtifactStat::from_zh_cn_raw("暴击伤+22.5%");
    assert!(result.is_some(), "应该能够成功解析'暴击伤+22.5%'");

    let stat = result.unwrap();
    assert_eq!(stat.name, ArtifactStatName::CriticalDamage);
    assert!((stat.value - 0.225).abs() < f64::EPSILON);
}

/// Test parsing of various critical damage formats
#[test]
fn test_critical_damage_parsing() {
    let test_cases = [("暴击伤+22.5%", 0.225), ("暴击伤+15.0%", 0.150), ("暴击伤害+30.2%", 0.302)];

    for (input, expected_value) in test_cases {
        let result = ArtifactStat::from_zh_cn_raw(input);
        assert!(result.is_some(), "应该能够成功解析'{input}'");

        let stat = result.unwrap();
        assert_eq!(stat.name, ArtifactStatName::CriticalDamage);
        assert!(
            (stat.value - expected_value).abs() < f64::EPSILON,
            "'{input}'的解析值应该是{expected_value:.3}，实际是{:.3}",
            stat.value
        );
    }
}
