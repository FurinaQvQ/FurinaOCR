use std::collections::HashSet;

use lazy_static::lazy_static;

// 原神角色名称集合，用于验证装备信息中的角色名称
lazy_static! {
    pub static ref CHARACTER_NAMES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        // 蒙德角色
        set.insert("安柏");
        set.insert("芭芭拉");
        set.insert("班尼特");
        set.insert("迪卢克");
        set.insert("菲谢尔");
        set.insert("凯亚");
        set.insert("丽莎");
        set.insert("米卡");
        set.insert("诺艾尔");
        set.insert("罗莎莉亚");
        set.insert("砂糖");
        set.insert("温迪");
        set.insert("优菈");
        set.insert("可莉");
        set.insert("琴");
        set.insert("迪奥娜");
        set.insert("雷泽");
        set.insert("莫娜");
        set.insert("阿贝多");
        set.insert("埃洛伊");
        set.insert("米卡");
        set.insert("菲米尼");
        set.insert("林尼");
        set.insert("琳妮特");

        // 璃月角色
        set.insert("北斗");
        set.insert("凝光");
        set.insert("重云");
        set.insert("甘雨");
        set.insert("胡桃");
        set.insert("刻晴");
        set.insert("七七");
        set.insert("申鹤");
        set.insert("辛焱");
        set.insert("行秋");
        set.insert("香菱");
        set.insert("魈");
        set.insert("烟绯");
        set.insert("夜兰");
        set.insert("云堇");
        set.insert("钟离");
        set.insert("白术");
        set.insert("闲云");
        set.insert("千织");

        // 稻妻角色
        set.insert("枫原万叶");
        set.insert("九条裟罗");
        set.insert("荒泷一斗");
        set.insert("久岐忍");
        set.insert("神里绫华");
        set.insert("神里绫人");
        set.insert("托马");
        set.insert("五郎");
        set.insert("宵宫");
        set.insert("早柚");
        set.insert("八重神子");
        set.insert("鹿野院平藏");
        set.insert("绮良良");
        set.insert("莱欧斯利");
        set.insert("菲米尼");
        set.insert("那维莱特");
        set.insert("芙宁娜");
        set.insert("夏洛蒂");

        // 须弥角色
        set.insert("坎蒂丝");
        set.insert("柯莱");
        set.insert("多莉");
        set.insert("纳西妲");
        set.insert("妮露");
        set.insert("赛诺");
        set.insert("提纳里");
        set.insert("迪希雅");
        set.insert("卡维");
        set.insert("莱依拉");
        set.insert("瑶瑶");
        set.insert("米卡");
        set.insert("绮良良");
        set.insert("莱欧斯利");
        set.insert("菲米尼");
        set.insert("那维莱特");
        set.insert("芙宁娜");
        set.insert("夏洛蒂");

        // 枫丹角色
        set.insert("林尼");
        set.insert("琳妮特");
        set.insert("菲米尼");
        set.insert("那维莱特");
        set.insert("芙宁娜");
        set.insert("夏洛蒂");
        set.insert("莱欧斯利");
        set.insert("千织");

        // 至冬角色
        set.insert("达达利亚");
        set.insert("罗莎莉亚");
        set.insert("米卡");
        set.insert("绮良良");
        set.insert("莱欧斯利");
        set.insert("菲米尼");
        set.insert("那维莱特");
        set.insert("芙宁娜");
        set.insert("夏洛蒂");

        // 坎瑞亚角色
        set.insert("凯亚");
        set.insert("戴因斯雷布");
        set.insert("米卡");
        set.insert("绮良良");
        set.insert("莱欧斯利");
        set.insert("菲米尼");
        set.insert("那维莱特");
        set.insert("芙宁娜");
        set.insert("夏洛蒂");

        // 其他角色
        set.insert("派蒙");
        set.insert("埃洛伊");
        set.insert("米卡");
        set.insert("绮良良");
        set.insert("莱欧斯利");
        set.insert("菲米尼");
        set.insert("那维莱特");
        set.insert("芙宁娜");
        set.insert("夏洛蒂");

        set
    };
}
