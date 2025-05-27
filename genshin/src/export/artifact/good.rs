use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::artifact::{
    ArtifactSetName, ArtifactSlot, ArtifactStat, ArtifactStatName, GenshinArtifact,
};

/// GOOD格式圣遗物导出模块
/// 用于将圣遗物数据转换为GOOD格式的JSON输出，支持与其他原神工具的数据交换
/// GOOD格式圣遗物包装结构体
///
/// 包装原始的 GenshinArtifact 结构体，提供GOOD格式的序列化实现。
/// 这种设计模式允许我们为同一个数据结构提供多种不同的序列化格式。
struct GOODArtifact<'a> {
    artifact: &'a GenshinArtifact,
}

/// 为 GOODArtifact 实现 Serialize trait
///
/// 将内部的圣遗物数据转换为GOOD格式的JSON结构。
/// GOOD格式的字段映射：
/// - setKey: 套装名称（英文）
/// - slotKey: 部位名称（英文）
/// - level: 强化等级
/// - rarity: 星级
/// - mainStatKey: 主属性名称（英文）
/// - location: 装备角色（英文名称）
/// - lock: 锁定状态
/// - substats: 副属性数组
impl<'a> Serialize for GOODArtifact<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let artifact = &self.artifact;

        // 收集所有非空的副属性
        let mut substats = Vec::new();
        if let Some(stat) = &artifact.sub_stat_1 {
            let good_stat = GOODStat::new(stat);
            substats.push(good_stat)
        }
        if let Some(stat) = &artifact.sub_stat_2 {
            let good_stat = GOODStat::new(stat);
            substats.push(good_stat)
        }
        if let Some(stat) = &artifact.sub_stat_3 {
            let good_stat = GOODStat::new(stat);
            substats.push(good_stat)
        }
        if let Some(stat) = &artifact.sub_stat_4 {
            let good_stat = GOODStat::new(stat);
            substats.push(good_stat)
        }

        // 序列化为GOOD格式的JSON对象
        let mut root = serializer.serialize_map(Some(8))?;
        root.serialize_entry("setKey", artifact.set_name.to_good())?;
        root.serialize_entry("slotKey", artifact.slot.to_good())?;
        root.serialize_entry("level", &artifact.level)?;
        root.serialize_entry("rarity", &artifact.star)?;
        root.serialize_entry("mainStatKey", artifact.main_stat.name.to_good())?;
        root.serialize_entry("location", equip_from_zh_cn(artifact.equip.clone().as_deref()))?;
        root.serialize_entry("lock", &artifact.lock)?;
        root.serialize_entry("substats", &substats)?;
        root.end()
    }
}

/// GOOD格式属性结构体
///
/// 表示GOOD格式中的一个属性，包含属性键名和数值。
/// 与内部格式不同，GOOD格式使用英文键名，并且百分比属性需要转换回百分比形式。
#[derive(Serialize)]
struct GOODStat<'a> {
    key: &'a str, // 属性键名（英文）
    value: f64,   // 属性数值
}

impl<'a> GOODStat<'a> {
    /// 从内部属性格式创建GOOD格式属性
    ///
    /// # 数值转换规则
    /// - 固定值属性（攻击力、生命值、防御力、元素精通）：保持原值
    /// - 百分比属性：转换回百分比形式（乘以100）
    ///
    /// # 参数
    /// - `stat`: 内部属性结构体引用
    ///
    /// # 返回值
    /// 返回GOOD格式的属性结构体
    fn new(stat: &ArtifactStat) -> GOODStat {
        GOODStat {
            key: stat.name.to_good(),
            value: match stat.name {
                // 固定值属性保持原值
                ArtifactStatName::Atk
                | ArtifactStatName::ElementalMastery
                | ArtifactStatName::Hp
                | ArtifactStatName::Def => stat.value,
                // 百分比属性转换为百分比形式
                _ => stat.value * 100.0,
            },
        }
    }
}

/// 为 ArtifactStatName 实现 GOOD 格式转换
///
/// 将中文属性名称转换为GOOD格式的英文键名。
/// 这些键名遵循GOOD标准，确保与其他工具的兼容性。
impl ArtifactStatName {
    /// 转换为GOOD格式的属性键名
    ///
    /// # 命名规则
    /// - 固定值属性：不带下划线（如 "atk", "hp", "def"）
    /// - 百分比属性：带下划线后缀（如 "atk_", "hp_", "def_"）
    /// - 元素伤害：格式为 "{element}_dmg_"
    /// - 特殊属性：使用缩写（如 "critRate_", "critDMG_", "eleMas"）
    pub fn to_good(&self) -> &'static str {
        match self {
            ArtifactStatName::HealingBonus => "heal_",
            ArtifactStatName::CriticalDamage => "critDMG_",
            ArtifactStatName::Critical => "critRate_",
            ArtifactStatName::Atk => "atk",
            ArtifactStatName::AtkPercentage => "atk_",
            ArtifactStatName::ElementalMastery => "eleMas",
            ArtifactStatName::Recharge => "enerRech_",
            ArtifactStatName::HpPercentage => "hp_",
            ArtifactStatName::Hp => "hp",
            ArtifactStatName::DefPercentage => "def_",
            ArtifactStatName::Def => "def",
            ArtifactStatName::ElectroBonus => "electro_dmg_",
            ArtifactStatName::PyroBonus => "pyro_dmg_",
            ArtifactStatName::HydroBonus => "hydro_dmg_",
            ArtifactStatName::CryoBonus => "cryo_dmg_",
            ArtifactStatName::AnemoBonus => "anemo_dmg_",
            ArtifactStatName::GeoBonus => "geo_dmg_",
            ArtifactStatName::PhysicalBonus => "physical_dmg_",
            ArtifactStatName::DendroBonus => "dendro_dmg_",
        }
    }
}

/// 为 ArtifactSlot 实现 GOOD 格式转换
///
/// 将中文部位名称转换为GOOD格式的英文键名。
impl ArtifactSlot {
    /// 转换为GOOD格式的部位键名
    ///
    /// # 部位映射
    /// - 生之花 → "flower"
    /// - 死之羽 → "plume"
    /// - 时之沙 → "sands"
    /// - 空之杯 → "goblet"
    /// - 理之冠 → "circlet"
    pub fn to_good(&self) -> &'static str {
        match self {
            ArtifactSlot::Flower => "flower",
            ArtifactSlot::Feather => "plume",
            ArtifactSlot::Sand => "sands",
            ArtifactSlot::Goblet => "goblet",
            ArtifactSlot::Head => "circlet",
        }
    }
}

/// 为 ArtifactSetName 实现 GOOD 格式转换
///
/// 将中文套装名称转换为GOOD格式的英文键名。
/// 这些键名与游戏内部数据和其他工具保持一致。
impl ArtifactSetName {
    /// 转换为GOOD格式的套装键名
    ///
    /// # 命名规则
    /// - 使用英文套装名称
    /// - 采用PascalCase命名风格
    /// - 保持与游戏内部数据的一致性
    /// - 确保与其他GOOD兼容工具的互操作性
    pub fn to_good(&self) -> &'static str {
        match self {
            ArtifactSetName::ArchaicPetra => "ArchaicPetra",
            ArtifactSetName::HeartOfDepth => "HeartOfDepth",
            ArtifactSetName::BlizzardStrayer => "BlizzardStrayer",
            ArtifactSetName::RetracingBolide => "RetracingBolide",
            ArtifactSetName::NoblesseOblige => "NoblesseOblige",
            ArtifactSetName::GladiatorFinale => "GladiatorsFinale",
            ArtifactSetName::MaidenBeloved => "MaidenBeloved",
            ArtifactSetName::ViridescentVenerer => "ViridescentVenerer",
            ArtifactSetName::LavaWalker => "Lavawalker",
            ArtifactSetName::CrimsonWitch => "CrimsonWitchOfFlames",
            ArtifactSetName::ThunderSmoother => "Thundersoother",
            ArtifactSetName::ThunderingFury => "ThunderingFury",
            ArtifactSetName::BloodstainedChivalry => "BloodstainedChivalry",
            ArtifactSetName::WandererTroupe => "WanderersTroupe",
            ArtifactSetName::Scholar => "Scholar",
            ArtifactSetName::Gambler => "Gambler",
            ArtifactSetName::TinyMiracle => "TinyMiracle",
            ArtifactSetName::MartialArtist => "MartialArtist",
            ArtifactSetName::BraveHeart => "BraveHeart",
            ArtifactSetName::ResolutionOfSojourner => "ResolutionOfSojourner",
            ArtifactSetName::DefenderWill => "DefendersWill",
            ArtifactSetName::Berserker => "Berserker",
            ArtifactSetName::Instructor => "Instructor",
            ArtifactSetName::Exile => "TheExile",
            ArtifactSetName::Adventurer => "Adventurer",
            ArtifactSetName::LuckyDog => "LuckyDog",
            ArtifactSetName::TravelingDoctor => "TravelingDoctor",
            ArtifactSetName::PrayersForWisdom => "PrayersForWisdom",
            ArtifactSetName::PrayersToSpringtime => "PrayersToSpringtime",
            ArtifactSetName::PrayersForIllumination => "PrayersForIllumination",
            ArtifactSetName::PrayersForDestiny => "PrayersForDestiny",
            ArtifactSetName::PaleFlame => "PaleFlame",
            ArtifactSetName::TenacityOfTheMillelith => "TenacityOfTheMillelith",
            ArtifactSetName::EmblemOfSeveredFate => "EmblemOfSeveredFate",
            ArtifactSetName::ShimenawaReminiscence => "ShimenawasReminiscence",
            ArtifactSetName::HuskOfOpulentDreams => "HuskOfOpulentDreams",
            ArtifactSetName::OceanHuedClam => "OceanHuedClam",
            ArtifactSetName::VermillionHereafter => "VermillionHereafter",
            ArtifactSetName::EchoesOfAnOffering => "EchoesOfAnOffering",
            ArtifactSetName::DeepwoodMemories => "DeepwoodMemories",
            ArtifactSetName::GildedDreams => "GildedDreams",
            ArtifactSetName::FlowerOfParadiseLost => "FlowerOfParadiseLost",
            ArtifactSetName::DesertPavilionChronicle => "DesertPavilionChronicle",
            ArtifactSetName::NymphsDream => "NymphsDream",
            ArtifactSetName::VourukashasGlow => "VourukashasGlow",
            ArtifactSetName::MarechausseeHunter => "MarechausseeHunter",
            ArtifactSetName::GoldenTroupe => "GoldenTroupe",
            ArtifactSetName::SongOfDaysPast => "SongOfDaysPast",
            ArtifactSetName::NighttimeWhispersInTheEchoingWoods => {
                "NighttimeWhispersInTheEchoingWoods"
            },
            ArtifactSetName::FragmentOfHarmonicWhimsy => "FragmentOfHarmonicWhimsy",
            ArtifactSetName::UnfinishedReverie => "UnfinishedReverie",
            ArtifactSetName::ScrollOfTheHeroOfCinderCity => "ScrollOfTheHeroOfCinderCity",
            ArtifactSetName::ObsidianCodex => "ObsidianCodex",
            ArtifactSetName::LongNightsOath => "LongNightsOath",
            ArtifactSetName::FinaleOfTheDeepGalleries => "FinaleOfTheDeepGalleries",
        }
    }
}

/// 角色名称中英文转换函数
///
/// 将中文角色名称转换为GOOD格式的英文角色名称。
/// 这些名称与游戏内部数据和其他工具保持一致。
///
/// # 参数
/// - `equip`: 可选的中文角色名称
///
/// # 返回值
/// 返回对应的英文角色名称，如果未找到匹配则返回空字符串
///
/// # 命名规则
/// - 使用PascalCase命名风格
/// - 保持与游戏官方英文名称的一致性
/// - 对于复合名称，去除空格和特殊字符
fn equip_from_zh_cn(equip: Option<&str>) -> &'static str {
    match equip {
        // 火元素角色
        Some("迪卢克") => "Diluc",
        Some("可莉") => "Klee",
        Some("胡桃") => "HuTao",
        Some("宵宫") => "Yoimiya",
        Some("安柏") => "Amber",
        Some("班尼特") => "Bennett",
        Some("香菱") => "Xiangling",
        Some("辛焱") => "Xinyan",
        Some("烟绯") => "Yanfei",
        Some("托马") => "Thoma",
        Some("迪希雅") => "Dehya",
        Some("林尼") => "Lyney",
        Some("夏沃蕾") => "Chevreuse",
        Some("嘉明") => "Gaming",
        Some("阿蕾奇诺") => "Arlecchino",
        Some("玛薇卡") => "Mavuika",

        // 水元素角色
        Some("莫娜") => "Mona",
        Some("达达利亚") => "Tartaglia",
        Some("珊瑚宫心海") => "SangonomiyaKokomi",
        Some("神里绫人") => "KamisatoAyato",
        Some("夜兰") => "Yelan",
        Some("妮露") => "Nilou",
        Some("芭芭拉") => "Barbara",
        Some("行秋") => "Xingqiu",
        Some("坎蒂丝") => "Candace",
        Some("芙宁娜") => "Furina",
        Some("那维莱特") => "Neuvillette",
        Some("希格雯") => "Sigewinne",
        Some("玛拉妮") => "Mualani",
        Some("塔利雅") => "Dahlia",

        // 雷元素角色
        Some("刻晴") => "Keqing",
        Some("雷电将军") => "RaidenShogun",
        Some("八重神子") => "YaeMiko",
        Some("赛诺") => "Cyno",
        Some("北斗") => "Beidou",
        Some("丽莎") => "Lisa",
        Some("雷泽") => "Razor",
        Some("菲谢尔") => "Fischl",
        Some("九条裟罗") => "KujouSara",
        Some("久岐忍") => "KukiShinobu",
        Some("多莉") => "Dori",
        Some("赛索斯") => "Sethos",
        Some("克洛琳德") => "Clorinde",
        Some("欧洛伦") => "Ororon",
        Some("伊安珊") => "Iansan",
        Some("瓦雷莎") => "Varesa",

        // 冰元素角色
        Some("七七") => "Qiqi",
        Some("甘雨") => "Ganyu",
        Some("神里绫华") => "KamisatoAyaka",
        Some("优菈") => "Eula",
        Some("埃洛伊") => "Aloy",
        Some("申鹤") => "Shenhe",
        Some("凯亚") => "Kaeya",
        Some("重云") => "Chongyun",
        Some("迪奥娜") => "Diona",
        Some("罗莎莉亚") => "Rosaria",
        Some("莱依拉") => "Layla",
        Some("米卡") => "Mika",
        Some("菲米尼") => "Freminet",
        Some("娜维娅") => "Navia",
        Some("莱欧斯利") => "Wriothesley",
        Some("夏洛蒂") => "Charlotte",
        Some("茜特菈莉") => "Citlali",
        Some("爱可菲") => "Escoffier",
        Some("斯柯克") => "Skirk",

        // 风元素角色
        Some("琴") => "Jean",
        Some("温迪") => "Venti",
        Some("魈") => "Xiao",
        Some("旅行者") => "Traveler",
        Some("枫原万叶") => "KaedeharaKazuha",
        Some("流浪者") => "Wanderer",
        Some("砂糖") => "Sucrose",
        Some("早柚") => "Sayu",
        Some("鹿野院平藏") => "ShikanoinHeizou",
        Some("珐露珊") => "Faruzan",
        Some("琳妮特") => "Lynette",
        Some("闲云") => "Xianyun",
        Some("恰斯卡") => "Chasca",
        Some("蓝砚") => "LanYan",
        Some("梦见月瑞希") => "YumemizukiMizuki",
        Some("伊法") => "Ifa",

        // 岩元素角色
        Some("钟离") => "Zhongli",
        Some("阿贝多") => "Albedo",
        Some("荒泷一斗") => "AratakiItto",
        Some("诺艾尔") => "Noelle",
        Some("凝光") => "Ningguang",
        Some("云堇") => "YunJin",
        Some("五郎") => "Gorou",
        Some("千织") => "Chiori",
        Some("卡齐娜") => "Kachina",
        Some("希诺宁") => "Xilonen",

        // 草元素角色
        Some("提纳里") => "Tighnari",
        Some("纳西妲") => "Nahida",
        Some("柯莱") => "Collei",
        Some("白术") => "Baizhu",
        Some("卡维") => "Kaveh",
        Some("瑶瑶") => "Yaoyao",
        Some("艾尔海森") => "Alhaitham",
        Some("绮良良") => "Kirara",
        Some("艾梅莉埃") => "Emilie",
        Some("基尼奇") => "Kinich",
        _ => "",
    }
}

/// GOOD格式导出结构体
///
/// 用于将圣遗物数据转换为GOOD格式的JSON输出。
/// 包含格式标识、版本号、数据来源和圣遗物列表。
///
/// # 示例
///
/// ```rust
/// use genshin::export::artifact::good::GOODFormat;
/// use genshin::artifact::GenshinArtifact;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let artifacts = vec![/* 圣遗物数据 */];
///     let good_format = GOODFormat::new(&artifacts);
///     let json = serde_json::to_string_pretty(&good_format)?;
///     println!("{}", json);
///     Ok(())
/// }
/// ```
#[derive(Serialize)]
pub struct GOODFormat<'a> {
    format: &'a str,                  // 格式标识
    version: u32,                     // 版本号
    source: &'a str,                  // 数据来源
    artifacts: Vec<GOODArtifact<'a>>, // 圣遗物列表
}

impl<'a> GOODFormat<'a> {
    /// 创建新的GOOD格式导出结构
    ///
    /// # 参数
    /// - `results`: 圣遗物数组的引用
    ///
    /// # 返回值
    /// 返回包含所有圣遗物数据的GOOD格式结构体
    ///
    /// # 示例
    /// ```rust
    /// use genshin::export::artifact::good::GOODFormat;
    /// use genshin::artifact::GenshinArtifact;
    /// use anyhow::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let artifacts = vec![/* 圣遗物数据 */];
    ///     let good_format = GOODFormat::new(&artifacts);
    ///     let json = serde_json::to_string_pretty(&good_format)?;
    ///     println!("{}", json);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(results: &'a [GenshinArtifact]) -> GOODFormat<'a> {
        let artifacts: Vec<GOODArtifact<'a>> =
            results.iter().map(|artifact| GOODArtifact { artifact }).collect();
        GOODFormat { format: "GOOD", version: 1, source: "furina", artifacts }
    }
}
