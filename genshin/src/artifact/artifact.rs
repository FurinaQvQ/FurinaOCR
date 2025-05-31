use std::hash::{Hash, Hasher};

use furina_core::utils::string_optimizer::parse_stat_optimized;
use log::error;
use regex::Regex;

use crate::character::CHARACTER_NAMES;
use crate::scanner::GenshinArtifactScanResult;

/// 圣遗物属性名称枚举
#[derive(Debug, Hash, Clone, PartialEq, Eq, strum_macros::Display)]
#[strum(serialize_all = "PascalCase")]
pub enum ArtifactStatName {
    HealingBonus,     // 治疗加成
    CriticalDamage,   // 暴击伤害
    Critical,         // 暴击率
    Atk,              // 攻击力（固定值）
    AtkPercentage,    // 攻击力（百分比）
    ElementalMastery, // 元素精通
    Recharge,         // 元素充能效率
    HpPercentage,     // 生命值（百分比）
    Hp,               // 生命值（固定值）
    DefPercentage,    // 防御力（百分比）
    Def,              // 防御力（固定值）
    ElectroBonus,     // 雷元素伤害加成
    PyroBonus,        // 火元素伤害加成
    HydroBonus,       // 水元素伤害加成
    CryoBonus,        // 冰元素伤害加成
    AnemoBonus,       // 风元素伤害加成
    GeoBonus,         // 岩元素伤害加成
    PhysicalBonus,    // 物理伤害加成
    DendroBonus,      // 草元素伤害加成
}

/// 圣遗物部位枚举
#[derive(Debug, Hash, Clone, PartialEq, Eq, strum_macros::Display)]
#[strum(serialize_all = "PascalCase")]
pub enum ArtifactSlot {
    Flower,  // 生之花
    Feather, // 死之羽
    Sand,    // 时之沙
    Goblet,  // 空之杯
    Head,    // 理之冠
}

/// 圣遗物套装名称枚举
#[derive(Debug, Hash, Clone, PartialEq, Eq, strum_macros::Display)]
#[strum(serialize_all = "PascalCase")]
pub enum ArtifactSetName {
    ArchaicPetra,                       // 磐陀裂生之岩
    HeartOfDepth,                       // 沉沦之心
    BlizzardStrayer,                    // 冰风迷途的勇士
    RetracingBolide,                    // 逆飞的流星
    NoblesseOblige,                     // 昔日宗室之仪
    GladiatorFinale,                    // 角斗士的终幕礼
    MaidenBeloved,                      // 被怜爱的少女
    ViridescentVenerer,                 // 翠绿之影
    LavaWalker,                         // 渡过烈火的贤人
    CrimsonWitch,                       // 炽烈的炎之魔女
    ThunderSmoother,                    // 平息鸣雷的尊者
    ThunderingFury,                     // 如雷的盛怒
    BloodstainedChivalry,               // 染血的骑士道
    WandererTroupe,                     // 流浪大地的乐团
    Scholar,                            // 学士
    Gambler,                            // 赌徒
    TinyMiracle,                        // 奇迹
    MartialArtist,                      // 武人
    BraveHeart,                         // 勇士之心
    ResolutionOfSojourner,              // 行者之心
    DefenderWill,                       // 守护之心
    Berserker,                          // 战狂
    Instructor,                         // 教官
    Exile,                              // 流放者
    Adventurer,                         // 冒险家
    LuckyDog,                           // 幸运儿
    TravelingDoctor,                    // 游医
    PrayersForWisdom,                   // 祭礼套装
    PrayersToSpringtime,                // 祭礼套装
    PrayersForIllumination,             // 祭礼套装
    PrayersForDestiny,                  // 祭礼套装
    PaleFlame,                          // 苍白之火
    TenacityOfTheMillelith,             // 千岩牢固
    EmblemOfSeveredFate,                // 绝缘之旗印
    ShimenawaReminiscence,              // 追忆之注连
    HuskOfOpulentDreams,                // 华馆梦醒形骸记
    OceanHuedClam,                      // 海染砗磲
    VermillionHereafter,                // 辰砂往生录
    EchoesOfAnOffering,                 // 来歆余响
    DeepwoodMemories,                   // 深林的记忆
    GildedDreams,                       // 饰金之梦
    FlowerOfParadiseLost,               // 乐园遗落之花
    DesertPavilionChronicle,            // 沙上楼阁史话
    NymphsDream,                        // 水仙之梦
    VourukashasGlow,                    // 花海甘露之光
    MarechausseeHunter,                 // 逐影猎人
    GoldenTroupe,                       // 黄金剧团
    SongOfDaysPast,                     // 昔日宗室之仪
    NighttimeWhispersInTheEchoingWoods, // 夜色花园的回响
    FragmentOfHarmonicWhimsy,           // 谐律奇想断章
    UnfinishedReverie,                  // 未竟的遐思
    ScrollOfTheHeroOfCinderCity,        // 烬城勇者绘卷
    ObsidianCodex,                      // 黑曜秘典
    LongNightsOath,                     // 长夜之誓
    FinaleOfTheDeepGalleries,           // 深廊终曲
}

/// 圣遗物属性结构体
#[derive(Debug, Clone)]
pub struct ArtifactStat {
    pub name: ArtifactStatName, // 属性名称
    pub value: f64,             // 属性数值（百分比已转换为小数）
}

/// 原神圣遗物完整信息结构体
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct GenshinArtifact {
    pub set_name: ArtifactSetName,        // 套装名称
    pub slot: ArtifactSlot,               // 部位
    pub star: i32,                        // 星级
    pub lock: bool,                       // 锁定状态
    pub level: i32,                       // 强化等级
    pub main_stat: ArtifactStat,          // 主属性
    pub sub_stat_1: Option<ArtifactStat>, // 副属性1
    pub sub_stat_2: Option<ArtifactStat>, // 副属性2
    pub sub_stat_3: Option<ArtifactStat>, // 副属性3
    pub sub_stat_4: Option<ArtifactStat>, // 副属性4
    pub equip: Option<String>,            // 装备角色
}

impl Hash for ArtifactStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let v = (self.value * 1000.0) as i32;
        v.hash(state);
    }
}

impl PartialEq for ArtifactStat {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        let v1 = (self.value * 1000.0) as i32;
        let v2 = (other.value * 1000.0) as i32;

        v1 == v2
    }
}

impl Eq for ArtifactStat {}

impl ArtifactStatName {
    pub fn from_zh_cn(name: &str, is_percentage: bool) -> Option<ArtifactStatName> {
        match name {
            "治疗加成" => Some(ArtifactStatName::HealingBonus),
            "暴击伤害" | "暴击伤" => Some(ArtifactStatName::CriticalDamage),
            "暴击率" => Some(ArtifactStatName::Critical),
            "攻击力" => {
                if is_percentage {
                    Some(ArtifactStatName::AtkPercentage)
                } else {
                    Some(ArtifactStatName::Atk)
                }
            },
            "元素精通" => Some(ArtifactStatName::ElementalMastery),
            "元素充能效率" => Some(ArtifactStatName::Recharge),
            "生命值" => {
                if is_percentage {
                    Some(ArtifactStatName::HpPercentage)
                } else {
                    Some(ArtifactStatName::Hp)
                }
            },
            "防御力" => {
                if is_percentage {
                    Some(ArtifactStatName::DefPercentage)
                } else {
                    Some(ArtifactStatName::Def)
                }
            },
            "雷元素伤害加成" => Some(ArtifactStatName::ElectroBonus),
            "火元素伤害加成" => Some(ArtifactStatName::PyroBonus),
            "水元素伤害加成" => Some(ArtifactStatName::HydroBonus),
            "冰元素伤害加成" => Some(ArtifactStatName::CryoBonus),
            "风元素伤害加成" => Some(ArtifactStatName::AnemoBonus),
            "岩元素伤害加成" => Some(ArtifactStatName::GeoBonus),
            "草元素伤害加成" => Some(ArtifactStatName::DendroBonus),
            "物理伤害加成" => Some(ArtifactStatName::PhysicalBonus),
            _ => None,
        }
    }
}

impl ArtifactStat {
    pub fn from_zh_cn_raw(s: &str) -> Option<ArtifactStat> {
        // 尝试使用优化的解析器
        match parse_stat_optimized(s) {
            Ok((name, value, is_percentage)) => {
                // 将字符串名称转换为枚举
                if let Some(stat_name) = ArtifactStatName::from_zh_cn(&name, is_percentage) {
                    Some(ArtifactStat { name: stat_name, value })
                } else {
                    error!("未知属性名称: `{name}`");
                    None
                }
            },
            Err(e) => {
                error!("属性解析失败: `{s}`, 错误: {e}");
                // 回退到原始解析方法
                Self::parse_stat_fallback(s)
            },
        }
    }

    /// 回退解析方法（保持向后兼容）
    fn parse_stat_fallback(s: &str) -> Option<ArtifactStat> {
        let temp: Vec<&str> = s.split('+').collect();
        if temp.len() != 2 {
            return None;
        }

        let is_percentage = temp[1].contains('%');
        let stat_name = ArtifactStatName::from_zh_cn(temp[0], is_percentage)?;

        // 移除百分号和逗号，然后解析数值
        let re = Regex::new("[%,]").unwrap();
        let mut value = match re.replace_all(temp[1], "").parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                error!("属性解析失败: `{s}`");
                return None;
            },
        };

        // 百分比数值转换为小数形式
        if is_percentage {
            value /= 100.0;
        }

        Some(ArtifactStat { name: stat_name, value })
    }
}

impl TryFrom<&GenshinArtifactScanResult> for GenshinArtifact {
    type Error = ();

    fn try_from(value: &GenshinArtifactScanResult) -> Result<Self, Self::Error> {
        // 识别套装名称
        let set_name = ArtifactSetName::from_zh_cn(&value.name).ok_or(())?;
        // 识别圣遗物部位
        let slot = ArtifactSlot::from_zh_cn(&value.name).ok_or(())?;
        let star = value.star;
        let lock = value.lock;

        // 解析主属性
        let main_stat = ArtifactStat::from_zh_cn_raw(
            (value.main_stat_name.clone() + "+" + value.main_stat_value.as_str()).as_str(),
        )
        .ok_or(())?;

        // 解析副属性（可能为空）
        let sub1 = ArtifactStat::from_zh_cn_raw(&value.sub_stat[0]);
        let sub2 = ArtifactStat::from_zh_cn_raw(&value.sub_stat[1]);
        let sub3 = ArtifactStat::from_zh_cn_raw(&value.sub_stat[2]);
        let sub4 = ArtifactStat::from_zh_cn_raw(&value.sub_stat[3]);

        // 解析装备角色信息
        let equip = if value.equip.ends_with("已装备") {
            let chars = value.equip.chars().collect::<Vec<_>>();
            let equip_name = chars[..chars.len() - 3].iter().collect::<String>();

            // 验证角色名称是否在有效角色列表中
            if CHARACTER_NAMES.contains(equip_name.as_str()) {
                Some(equip_name)
            } else {
                None
            }
        } else {
            None
        };

        Ok(GenshinArtifact {
            set_name,
            slot,
            star,
            lock,
            level: value.level,
            main_stat,
            sub_stat_1: sub1,
            sub_stat_2: sub2,
            sub_stat_3: sub3,
            sub_stat_4: sub4,
            equip,
        })
    }
}

impl ArtifactSetName {
    pub fn from_zh_cn(s: &str) -> Option<ArtifactSetName> {
        match s {
            // 四星套装
            "战狂的蔷薇" | "战狂的翎羽" | "战狂的时计" | "战狂的骨杯" | "战狂的鬼面" => {
                Some(ArtifactSetName::Berserker)
            },
            "勇士的勋章" | "勇士的期许" | "勇士的坚毅" | "勇士的壮行" | "勇士的冠冕" => {
                Some(ArtifactSetName::BraveHeart)
            },
            "守护之花" | "守护徽印" | "守护座钟" | "守护之皿" | "守护束带" => {
                Some(ArtifactSetName::DefenderWill)
            },
            "流放者之花" | "流放者之羽" | "流放者怀表" | "流放者之杯" | "流放者头冠" => {
                Some(ArtifactSetName::Exile)
            },
            "赌徒的胸花" | "赌徒的羽饰" | "赌徒的怀表" | "赌徒的骰盅" | "赌徒的耳环" => {
                Some(ArtifactSetName::Gambler)
            },
            "教官的胸花" | "教官的羽饰" | "教官的怀表" | "教官的茶杯" | "教官的帽子" => {
                Some(ArtifactSetName::Instructor)
            },
            "武人的红花" | "武人的羽饰" | "武人的水漏" | "武人的酒杯" | "武人的头巾" => {
                Some(ArtifactSetName::MartialArtist)
            },
            "故人之心" | "归乡之羽" | "逐光之石" | "异国之盏" | "感别之冠" => {
                Some(ArtifactSetName::ResolutionOfSojourner)
            },
            "学士的书签" | "学士的羽笔" | "学士的时钟" | "学士的墨杯" | "学士的镜片" => {
                Some(ArtifactSetName::Scholar)
            },
            "奇迹之花" | "奇迹之羽" | "奇迹之沙" | "奇迹之杯" | "奇迹耳坠" => {
                Some(ArtifactSetName::TinyMiracle)
            },

            // 三星套装
            "冒险家之花" | "冒险家尾羽" | "冒险家怀表" | "冒险家金杯" | "冒险家头带" => {
                Some(ArtifactSetName::Adventurer)
            },
            "幸运儿绿花" | "幸运儿鹰羽" | "幸运儿沙漏" | "幸运儿之杯" | "幸运儿银冠" => {
                Some(ArtifactSetName::LuckyDog)
            },
            "游医的银莲" | "游医的枭羽" | "游医的怀钟" | "游医的药壶" | "游医的方巾" => {
                Some(ArtifactSetName::TravelingDoctor)
            },

            // 祭礼系列
            "祭雷礼冠" => Some(ArtifactSetName::PrayersForWisdom),
            "祭冰礼冠" => Some(ArtifactSetName::PrayersToSpringtime),
            "祭火礼冠" => Some(ArtifactSetName::PrayersForIllumination),
            "祭水礼冠" => Some(ArtifactSetName::PrayersForDestiny),

            // 1.0版本套装
            "角斗士的留恋" | "角斗士的归宿" | "角斗士的希冀" | "角斗士的酣醉" | "角斗士的凯旋" => {
                Some(ArtifactSetName::GladiatorFinale)
            },
            "乐团的晨光" | "琴师的箭羽" | "终幕的时计" | "吟游者之壶" | "指挥的礼帽" => {
                Some(ArtifactSetName::WandererTroupe)
            },
            "野花记忆的绿野"
            | "猎人青翠的箭羽"
            | "翠绿猎人的笃定"
            | "翠绿猎人的容器"
            | "翠绿的猎人之冠" => Some(ArtifactSetName::ViridescentVenerer),
            "远方的少女之心"
            | "少女飘摇的思念"
            | "少女苦短的良辰"
            | "少女片刻的闲暇"
            | "少女易逝的芳颜" => Some(ArtifactSetName::MaidenBeloved),
            "魔女的炎之花" | "魔女常燃之羽" | "魔女破灭之时" | "魔女的心之火" | "焦灼的魔女帽" => {
                Some(ArtifactSetName::CrimsonWitch)
            },
            "雷鸟的怜悯" | "雷灾的孑遗" | "雷霆的时计" | "降雷的凶兆" | "唤雷的头冠" => {
                Some(ArtifactSetName::ThunderingFury)
            },
            "平雷之心" | "平雷之羽" | "平雷之刻" | "平雷之器" | "平雷之冠" => {
                Some(ArtifactSetName::ThunderSmoother)
            },
            "渡火者的决绝" | "渡火者的解脱" | "渡火者的煎熬" | "渡火者的醒悟" | "渡火者的智慧" => {
                Some(ArtifactSetName::LavaWalker)
            },
            "染血的铁之心" | "染血的黑之羽" | "骑士染血之时" | "染血骑士之杯" | "染血的铁假面" => {
                Some(ArtifactSetName::BloodstainedChivalry)
            },
            "宗室之花" | "宗室之翎" | "宗室时计" | "宗室银瓮" | "宗室面具" => {
                Some(ArtifactSetName::NoblesseOblige)
            },
            "磐陀裂生之花" | "嵯峨群峰之翼" | "星罗圭壁之晷" | "星罗圭璧之晷" | "巉岩琢塑之樽"
            | "不动玄石之相" => Some(ArtifactSetName::ArchaicPetra),
            "夏祭之花" | "夏祭终末" | "夏祭之刻" | "夏祭水玉" | "夏祭之面" => {
                Some(ArtifactSetName::RetracingBolide)
            },

            // 1.2版本套装
            "历经风雪的思念"
            | "摧冰而行的执望"
            | "冰雪故园的终期"
            | "遍结寒霜的傲骨"
            | "破冰踏雪的回音" => Some(ArtifactSetName::BlizzardStrayer),
            "饰金胸花" | "追忆之风" | "坚铜罗盘" | "沉波之盏" | "酒渍船帽" => {
                Some(ArtifactSetName::HeartOfDepth)
            },

            // 2.0版本套装
            "无垢之花" | "贤医之羽" | "停摆之刻" | "超越之盏" | "嗤笑之面" => {
                Some(ArtifactSetName::PaleFlame)
            },
            "勋绩之花" | "昭武翎羽" | "金铜时晷" | "盟誓金爵" | "将帅兜鍪" => {
                Some(ArtifactSetName::TenacityOfTheMillelith)
            },
            "明威之镡" | "切落之羽" | "雷云之笼" | "绯花之壶" | "华饰之兜" => {
                Some(ArtifactSetName::EmblemOfSeveredFate)
            },
            "羁缠之花" | "思忆之矢" | "朝露之时" | "祈望之心" | "无常之面" => {
                Some(ArtifactSetName::ShimenawaReminiscence)
            },

            // 2.1版本套装
            "荣花之期" | "华馆之羽" | "众生之谣" | "梦醒之瓢" | "形骸之笠" => {
                Some(ArtifactSetName::HuskOfOpulentDreams)
            },
            "海染之花" | "渊宫之羽" | "离别之贝" | "真珠之笼" | "海祇之冠" => {
                Some(ArtifactSetName::OceanHuedClam)
            },

            // 2.6版本套装
            "生灵之华" | "潜光片羽" | "阳辔之遗" | "结契之刻" | "虺雷之姿" => {
                Some(ArtifactSetName::VermillionHereafter)
            },
            "魂香之花" | "垂玉之叶" | "祝祀之凭" | "涌泉之盏" | "浮溯之珏" => {
                Some(ArtifactSetName::EchoesOfAnOffering)
            },

            // 3.0版本套装
            "迷宫的游人" | "翠蔓的智者" | "贤智的定期" | "迷误者之灯" | "月桂的宝冠" => {
                Some(ArtifactSetName::DeepwoodMemories)
            },
            "梦中的铁花" | "裁断的翎羽" | "沉金的岁月" | "如蜜的终宴" | "沙王的投影" => {
                Some(ArtifactSetName::GildedDreams)
            },

            // 3.2版本套装
            "月女的华彩" | "谢落的筵席" | "凝结的时刻" | "守秘的魔瓶" | "紫晶的花冠" => {
                Some(ArtifactSetName::FlowerOfParadiseLost)
            },
            "众王之都的开端"
            | "黄金邦国的结末"
            | "失落迷途的机芯"
            | "迷醉长梦的守护"
            | "流沙贵嗣的遗宝" => Some(ArtifactSetName::DesertPavilionChronicle),

            // 3.6版本套装
            "旅途中的鲜花"
            | "坏巫师的羽杖"
            | "水仙的时时刻刻"
            | "勇者们的茶会"
            | "恶龙的单片镜" => Some(ArtifactSetName::NymphsDream),
            "灵光源起之蕊" | "琦色灵彩之羽" | "久远花落之时" | "无边酣乐之筵" | "灵光明烁之心" => {
                Some(ArtifactSetName::VourukashasGlow)
            },

            // 4.0版本套装
            "猎人的胸花" | "杰作的序曲" | "裁判的时刻" | "遗忘的容器" | "老兵的容颜" => {
                Some(ArtifactSetName::MarechausseeHunter)
            },
            "黄金乐曲的变奏"
            | "黄金飞鸟的落羽"
            | "黄金时代的先声"
            | "黄金之夜的喧嚣"
            | "黄金剧团的奖赏" => Some(ArtifactSetName::GoldenTroupe),

            // 4.3版本套装
            "昔时遗落之誓" | "昔时浮想之思" | "昔时回映之音" | "昔时应许之梦" | "昔时传奏之诗" => {
                Some(ArtifactSetName::SongOfDaysPast)
            },
            "无私的妆饰花" | "诚恳的蘸水笔" | "忠实的砂时计" | "慷慨的墨水瓶" | "慈爱的淑女帽" => {
                Some(ArtifactSetName::NighttimeWhispersInTheEchoingWoods)
            },

            // 4.6版本套装
            "谐律交响的前奏"
            | "古海玄幽的夜想"
            | "命途轮转的谐谑"
            | "灵露倾洒的狂诗"
            | "异想零落的圆舞" => Some(ArtifactSetName::FragmentOfHarmonicWhimsy),
            "暗结的明花" | "褪光的翠尾" | "举业的识刻" | "筹谋的共樽" | "失冕的宝冠" => {
                Some(ArtifactSetName::UnfinishedReverie)
            },

            // 5.0版本套装
            "驯兽师的护符" | "巡山客的信标" | "秘术家的金盘" | "游学者的爪杯" | "魔战士的羽面" => {
                Some(ArtifactSetName::ScrollOfTheHeroOfCinderCity)
            },
            "异种的期许" | "灵髓的根脉" | "夜域的迷思" | "纷争的前宴" | "诸圣的礼冠" => {
                Some(ArtifactSetName::ObsidianCodex)
            },

            // 5.5版本套装
            "执灯人的誓言" | "夜鸣莺的尾羽" | "不死者的哀铃" | "未吹响的号角" | "被浸染的缨盔" => {
                Some(ArtifactSetName::LongNightsOath)
            },
            "深廊的回奏之歌"
            | "深廊的漫远之约"
            | "深廊的湮落之刻"
            | "深廊的饫赐之宴"
            | "深廊的遂失之冕" => Some(ArtifactSetName::FinaleOfTheDeepGalleries),

            _ => None,
        }
    }
}

impl ArtifactSlot {
    pub fn from_zh_cn(s: &str) -> Option<ArtifactSlot> {
        match s {
            // 生之花（Flower）
            "磐陀裂生之花"
            | "历经风雪的思念"
            | "染血的铁之心"
            | "魔女的炎之花"
            | "角斗士的留恋"
            | "饰金胸花"
            | "渡火者的决绝"
            | "远方的少女之心"
            | "宗室之花"
            | "夏祭之花"
            | "平雷之心"
            | "雷鸟的怜悯"
            | "野花记忆的绿野"
            | "乐团的晨光"
            | "战狂的蔷薇"
            | "勇士的勋章"
            | "守护之花"
            | "流放者之花"
            | "赌徒的胸花"
            | "教官的胸花"
            | "武人的红花"
            | "故人之心"
            | "学士的书签"
            | "奇迹之花"
            | "冒险家之花"
            | "幸运儿绿花"
            | "游医的银莲"
            | "勋绩之花"
            | "无垢之花"
            | "明威之镡"
            | "羁缠之花"
            | "荣花之期"
            | "海染之花"
            | "生灵之华"
            | "魂香之花"
            | "迷宫的游人"
            | "梦中的铁花"
            | "月女的华彩"
            | "众王之都的开端"
            | "旅途中的鲜花"
            | "灵光源起之蕊"
            | "猎人的胸花"
            | "黄金乐曲的变奏"
            | "昔时遗落之誓"
            | "无私的妆饰花"
            | "谐律交响的前奏"
            | "暗结的明花"
            | "驯兽师的护符"
            | "异种的期许"
            | "执灯人的誓言"
            | "深廊的回奏之歌" => Some(ArtifactSlot::Flower),

            // 死之羽（Feather）
            "嵯峨群峰之翼"
            | "摧冰而行的执望"
            | "染血的黑之羽"
            | "魔女常燃之羽"
            | "角斗士的归宿"
            | "追忆之风"
            | "渡火者的解脱"
            | "少女飘摇的思念"
            | "宗室之翎"
            | "夏祭终末"
            | "平雷之羽"
            | "雷灾的孑遗"
            | "猎人青翠的箭羽"
            | "琴师的箭羽"
            | "战狂的翎羽"
            | "勇士的期许"
            | "守护徽印"
            | "流放者之羽"
            | "赌徒的羽饰"
            | "教官的羽饰"
            | "武人的羽饰"
            | "归乡之羽"
            | "学士的羽笔"
            | "奇迹之羽"
            | "冒险家尾羽"
            | "幸运儿鹰羽"
            | "游医的枭羽"
            | "昭武翎羽"
            | "贤医之羽"
            | "切落之羽"
            | "思忆之矢"
            | "华馆之羽"
            | "渊宫之羽"
            | "潜光片羽"
            | "垂玉之叶"
            | "翠蔓的智者"
            | "裁断的翎羽"
            | "谢落的筵席"
            | "黄金邦国的结末"
            | "坏巫师的羽杖"
            | "琦色灵彩之羽"
            | "杰作的序曲"
            | "黄金飞鸟的落羽"
            | "昔时浮想之思"
            | "诚恳的蘸水笔"
            | "古海玄幽的夜想"
            | "褪光的翠尾"
            | "巡山客的信标"
            | "灵髓的根脉"
            | "夜鸣莺的尾羽"
            | "深廊的漫远之约" => Some(ArtifactSlot::Feather),

            // 时之沙（Sand）
            "星罗圭壁之晷"
            | "星罗圭璧之晷"
            | "冰雪故园的终期"
            | "骑士染血之时"
            | "魔女破灭之时"
            | "角斗士的希冀"
            | "坚铜罗盘"
            | "渡火者的煎熬"
            | "少女苦短的良辰"
            | "宗室时计"
            | "夏祭之刻"
            | "平雷之刻"
            | "雷霆的时计"
            | "翠绿猎人的笃定"
            | "终幕的时计"
            | "终末的时计"
            | "战狂的时计"
            | "勇士的坚毅"
            | "守护座钟"
            | "流放者怀表"
            | "赌徒的怀表"
            | "教官的怀表"
            | "武人的水漏"
            | "逐光之石"
            | "学士的时钟"
            | "奇迹之沙"
            | "冒险家怀表"
            | "幸运儿沙漏"
            | "游医的怀钟"
            | "金铜时晷"
            | "停摆之刻"
            | "雷云之笼"
            | "朝露之时"
            | "众生之谣"
            | "离别之贝"
            | "阳辔之遗"
            | "祝祀之凭"
            | "贤智的定期"
            | "沉金的岁月"
            | "凝结的时刻"
            | "失落迷途的机芯"
            | "水仙的时时刻刻"
            | "久远花落之时"
            | "裁判的时刻"
            | "黄金时代的先声"
            | "昔时回映之音"
            | "忠实的砂时计"
            | "命途轮转的谐谑"
            | "举业的识刻"
            | "秘术家的金盘"
            | "夜域的迷思"
            | "不死者的哀铃"
            | "深廊的湮落之刻" => Some(ArtifactSlot::Sand),

            // 空之杯（Goblet）
            "巉岩琢塑之樽"
            | "遍结寒霜的傲骨"
            | "染血骑士之杯"
            | "魔女的心之火"
            | "角斗士的酣醉"
            | "沉波之盏"
            | "渡火者的醒悟"
            | "少女片刻的闲暇"
            | "宗室银瓮"
            | "夏祭水玉"
            | "平雷之器"
            | "降雷的凶兆"
            | "翠绿猎人的容器"
            | "吟游者之壶"
            | "战狂的骨杯"
            | "勇士的壮行"
            | "守护之皿"
            | "流放者之杯"
            | "赌徒的骰盅"
            | "教官的茶杯"
            | "武人的酒杯"
            | "异国之盏"
            | "学士的墨杯"
            | "奇迹之杯"
            | "冒险家金杯"
            | "幸运儿之杯"
            | "游医的药壶"
            | "盟誓金爵"
            | "超越之盏"
            | "绯花之壶"
            | "祈望之心"
            | "梦醒之瓢"
            | "真珠之笼"
            | "结契之刻"
            | "涌泉之盏"
            | "迷误者之灯"
            | "如蜜的终宴"
            | "守秘的魔瓶"
            | "迷醉长梦的守护"
            | "勇者们的茶会"
            | "无边酣乐之筵"
            | "遗忘的容器"
            | "黄金之夜的喧嚣"
            | "昔时应许之梦"
            | "慷慨的墨水瓶"
            | "灵露倾洒的狂诗"
            | "筹谋的共樽"
            | "游学者的爪杯"
            | "纷争的前宴"
            | "未吹响的号角"
            | "深廊的饫赐之宴" => Some(ArtifactSlot::Goblet),

            // 理之冠（Head）
            "不动玄石之相"
            | "破冰踏雪的回音"
            | "染血的铁假面"
            | "焦灼的魔女帽"
            | "角斗士的凯旋"
            | "酒渍船帽"
            | "渡火者的智慧"
            | "少女易逝的芳颜"
            | "宗室面具"
            | "夏祭之面"
            | "平雷之冠"
            | "唤雷的头冠"
            | "翠绿的猎人之冠"
            | "指挥的礼帽"
            | "战狂的鬼面"
            | "勇士的冠冕"
            | "守护束带"
            | "流放者头冠"
            | "赌徒的耳环"
            | "教官的帽子"
            | "武人的头巾"
            | "感别之冠"
            | "学士的镜片"
            | "奇迹耳坠"
            | "冒险家头带"
            | "幸运儿银冠"
            | "游医的方巾"
            | "将帅兜鍪"
            | "嗤笑之面"
            | "华饰之兜"
            | "无常之面"
            | "形骸之笠"
            | "海祇之冠"
            | "虺雷之姿"
            | "浮溯之珏"
            | "月桂的宝冠"
            | "沙王的投影"
            | "紫晶的花冠"
            | "流沙贵嗣的遗宝"
            | "恶龙的单片镜"
            | "灵光明烁之心"
            | "老兵的容颜"
            | "黄金剧团的奖赏"
            | "昔时传奏之诗"
            | "慈爱的淑女帽"
            | "异想零落的圆舞"
            | "失冕的宝冠"
            | "魔战士的羽面"
            | "诸圣的礼冠"
            | "被浸染的缨盔"
            | "深廊的遂失之冕" => Some(ArtifactSlot::Head),

            // 祭礼系列套装（特殊）
            "祭水礼冠" | "祭火礼冠" | "祭雷礼冠" | "祭冰礼冠" => {
                Some(ArtifactSlot::Head)
            },

            // 未知部位
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_stat_name_from_zh_cn() {
        // 测试百分比属性
        assert_eq!(
            ArtifactStatName::from_zh_cn("攻击力", true),
            Some(ArtifactStatName::AtkPercentage)
        );
        assert_eq!(
            ArtifactStatName::from_zh_cn("生命值", true),
            Some(ArtifactStatName::HpPercentage)
        );
        assert_eq!(
            ArtifactStatName::from_zh_cn("防御力", true),
            Some(ArtifactStatName::DefPercentage)
        );

        // 测试固定值属性
        assert_eq!(ArtifactStatName::from_zh_cn("攻击力", false), Some(ArtifactStatName::Atk));
        assert_eq!(ArtifactStatName::from_zh_cn("生命值", false), Some(ArtifactStatName::Hp));
        assert_eq!(ArtifactStatName::from_zh_cn("防御力", false), Some(ArtifactStatName::Def));

        // 测试其他属性
        assert_eq!(ArtifactStatName::from_zh_cn("暴击率", false), Some(ArtifactStatName::Critical));
        assert_eq!(
            ArtifactStatName::from_zh_cn("暴击伤害", false),
            Some(ArtifactStatName::CriticalDamage)
        );
        assert_eq!(
            ArtifactStatName::from_zh_cn("元素精通", false),
            Some(ArtifactStatName::ElementalMastery)
        );
        assert_eq!(
            ArtifactStatName::from_zh_cn("元素充能效率", false),
            Some(ArtifactStatName::Recharge)
        );

        // 测试元素伤害加成
        assert_eq!(
            ArtifactStatName::from_zh_cn("火元素伤害加成", false),
            Some(ArtifactStatName::PyroBonus)
        );
        assert_eq!(
            ArtifactStatName::from_zh_cn("水元素伤害加成", false),
            Some(ArtifactStatName::HydroBonus)
        );
        assert_eq!(
            ArtifactStatName::from_zh_cn("雷元素伤害加成", false),
            Some(ArtifactStatName::ElectroBonus)
        );

        // 测试无效输入
        assert_eq!(ArtifactStatName::from_zh_cn("无效属性", false), None);
    }

    #[test]
    fn test_artifact_stat_from_zh_cn_raw() {
        // 测试百分比属性解析
        let stat = ArtifactStat::from_zh_cn_raw("攻击力+46.6%").unwrap();
        assert_eq!(stat.name, ArtifactStatName::AtkPercentage);
        assert!((stat.value - 0.466).abs() < 0.001);

        // 测试固定值属性解析
        let stat = ArtifactStat::from_zh_cn_raw("攻击力+311").unwrap();
        assert_eq!(stat.name, ArtifactStatName::Atk);
        assert!((stat.value - 311.0).abs() < 0.001);

        // 测试暴击率
        let stat = ArtifactStat::from_zh_cn_raw("暴击率+6.2%").unwrap();
        assert_eq!(stat.name, ArtifactStatName::Critical);
        assert!((stat.value - 0.062).abs() < 0.001);

        // 测试元素精通
        let stat = ArtifactStat::from_zh_cn_raw("元素精通+187").unwrap();
        assert_eq!(stat.name, ArtifactStatName::ElementalMastery);
        assert!((stat.value - 187.0).abs() < 0.001);

        // 测试无效格式
        assert!(ArtifactStat::from_zh_cn_raw("无效格式").is_none());
        assert!(ArtifactStat::from_zh_cn_raw("攻击力").is_none());
    }

    #[test]
    fn test_artifact_stat_equality() {
        let stat1 = ArtifactStat { name: ArtifactStatName::Critical, value: 0.062 };
        let stat2 = ArtifactStat {
            name: ArtifactStatName::Critical,
            value: 0.0621, // 略微不同的值
        };
        let stat3 = ArtifactStat { name: ArtifactStatName::CriticalDamage, value: 0.062 };

        // 相同名称和相近值应该相等（精度到千分位）
        assert_eq!(stat1, stat2);

        // 不同名称应该不相等
        assert_ne!(stat1, stat3);
    }

    #[test]
    fn test_artifact_set_name_from_zh_cn() {
        // 测试常见套装（使用具体的圣遗物名称）
        assert_eq!(
            ArtifactSetName::from_zh_cn("魔女的炎之花"),
            Some(ArtifactSetName::CrimsonWitch)
        );
        assert_eq!(
            ArtifactSetName::from_zh_cn("野花记忆的绿野"),
            Some(ArtifactSetName::ViridescentVenerer)
        );
        assert_eq!(
            ArtifactSetName::from_zh_cn("明威之镡"),
            Some(ArtifactSetName::EmblemOfSeveredFate)
        );
        assert_eq!(ArtifactSetName::from_zh_cn("饰金胸花"), Some(ArtifactSetName::HeartOfDepth));

        // 测试新套装
        assert_eq!(
            ArtifactSetName::from_zh_cn("黄金乐曲的变奏"),
            Some(ArtifactSetName::GoldenTroupe)
        );
        assert_eq!(
            ArtifactSetName::from_zh_cn("猎人的胸花"),
            Some(ArtifactSetName::MarechausseeHunter)
        );

        // 测试无效输入
        assert_eq!(ArtifactSetName::from_zh_cn("无效套装"), None);
    }

    #[test]
    fn test_artifact_slot_from_zh_cn() {
        // 测试生之花
        assert_eq!(ArtifactSlot::from_zh_cn("魔女的炎之花"), Some(ArtifactSlot::Flower));
        assert_eq!(ArtifactSlot::from_zh_cn("野花记忆的绿野"), Some(ArtifactSlot::Flower));

        // 测试死之羽
        assert_eq!(ArtifactSlot::from_zh_cn("魔女常燃之羽"), Some(ArtifactSlot::Feather));
        assert_eq!(ArtifactSlot::from_zh_cn("猎人青翠的箭羽"), Some(ArtifactSlot::Feather));

        // 测试时之沙
        assert_eq!(ArtifactSlot::from_zh_cn("魔女破灭之时"), Some(ArtifactSlot::Sand));
        assert_eq!(ArtifactSlot::from_zh_cn("翠绿猎人的笃定"), Some(ArtifactSlot::Sand));

        // 测试空之杯
        assert_eq!(ArtifactSlot::from_zh_cn("魔女的心之火"), Some(ArtifactSlot::Goblet));
        assert_eq!(ArtifactSlot::from_zh_cn("翠绿猎人的容器"), Some(ArtifactSlot::Goblet));

        // 测试理之冠
        assert_eq!(ArtifactSlot::from_zh_cn("焦灼的魔女帽"), Some(ArtifactSlot::Head));
        assert_eq!(ArtifactSlot::from_zh_cn("翠绿的猎人之冠"), Some(ArtifactSlot::Head));

        // 测试无效输入
        assert_eq!(ArtifactSlot::from_zh_cn("无效部位"), None);
    }

    #[test]
    fn test_artifact_stat_display() {
        let stat = ArtifactStat { name: ArtifactStatName::Critical, value: 0.062 };

        // 测试Display trait实现
        assert_eq!(format!("{}", stat.name), "Critical");
    }

    #[test]
    fn test_artifact_slot_display() {
        assert_eq!(format!("{}", ArtifactSlot::Flower), "Flower");
        assert_eq!(format!("{}", ArtifactSlot::Feather), "Feather");
        assert_eq!(format!("{}", ArtifactSlot::Sand), "Sand");
        assert_eq!(format!("{}", ArtifactSlot::Goblet), "Goblet");
        assert_eq!(format!("{}", ArtifactSlot::Head), "Head");
    }

    #[test]
    fn test_artifact_set_name_display() {
        assert_eq!(format!("{}", ArtifactSetName::CrimsonWitch), "CrimsonWitch");
        assert_eq!(format!("{}", ArtifactSetName::ViridescentVenerer), "ViridescentVenerer");
        assert_eq!(format!("{}", ArtifactSetName::EmblemOfSeveredFate), "EmblemOfSeveredFate");
    }

    #[test]
    fn test_genshin_artifact_creation() {
        let main_stat = ArtifactStat { name: ArtifactStatName::AtkPercentage, value: 0.466 };

        let sub_stat = ArtifactStat { name: ArtifactStatName::Critical, value: 0.062 };

        let artifact = GenshinArtifact {
            set_name: ArtifactSetName::CrimsonWitch,
            slot: ArtifactSlot::Sand,
            star: 5,
            lock: false,
            level: 20,
            main_stat: main_stat.clone(),
            sub_stat_1: Some(sub_stat.clone()),
            sub_stat_2: None,
            sub_stat_3: None,
            sub_stat_4: None,
            equip: Some("迪卢克".to_string()),
        };

        assert_eq!(artifact.set_name, ArtifactSetName::CrimsonWitch);
        assert_eq!(artifact.slot, ArtifactSlot::Sand);
        assert_eq!(artifact.star, 5);
        assert_eq!(artifact.level, 20);
        assert_eq!(artifact.main_stat, main_stat);
        assert_eq!(artifact.sub_stat_1, Some(sub_stat));
        assert_eq!(artifact.equip, Some("迪卢克".to_string()));
    }

    #[test]
    fn test_artifact_hash_and_equality() {
        let stat1 = ArtifactStat { name: ArtifactStatName::Critical, value: 0.062 };
        let stat2 = ArtifactStat {
            name: ArtifactStatName::Critical,
            value: 0.0621, // 略微不同但在精度范围内
        };

        let artifact1 = GenshinArtifact {
            set_name: ArtifactSetName::CrimsonWitch,
            slot: ArtifactSlot::Sand,
            star: 5,
            lock: false,
            level: 20,
            main_stat: stat1.clone(),
            sub_stat_1: Some(stat1.clone()),
            sub_stat_2: None,
            sub_stat_3: None,
            sub_stat_4: None,
            equip: None,
        };

        let artifact2 = GenshinArtifact {
            set_name: ArtifactSetName::CrimsonWitch,
            slot: ArtifactSlot::Sand,
            star: 5,
            lock: false,
            level: 20,
            main_stat: stat2.clone(),
            sub_stat_1: Some(stat2.clone()),
            sub_stat_2: None,
            sub_stat_3: None,
            sub_stat_4: None,
            equip: None,
        };

        // 测试相等性（应该相等，因为数值差异在精度范围内）
        assert_eq!(artifact1, artifact2);

        // 测试哈希一致性
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        artifact1.hash(&mut hasher1);
        artifact2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
