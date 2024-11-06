/// 定义区域代码与名称的映射为常量
const REGION_MAP: &[(&str, &str)] = &[
    ("SJC", "美国圣何塞"),
    ("ARN", "瑞典阿尔斯塔"),
    ("IAD", "美国华盛顿"),
    ("SEA", "美国西雅图"),
    ("NRT", "日本成田"),
    ("LHR", "英国伦敦"),
    ("BOM", "印度孟买"),
    ("ORD", "美国芝加哥"),
    ("YUL", "加拿大蒙特利尔"),
    ("YYZ", "加拿大多伦多"),
    ("ZRH", "瑞士苏黎世"),
    ("SIN", "新加坡"),
    ("SYD", "澳大利亚悉尼"),
    ("HKG", "中国香港"),
    ("TYO", "日本东京"),
    ("ICN", "韩国仁川"),
    ("KUL", "马来西亚吉隆坡"),
    ("MXP", "意大利米兰"),
    ("FRA", "德国法兰克福"),
    ("AMS", "荷兰阿姆斯特丹"),
    ("LAX", "美国洛杉矶"),
    ("GRU", "巴西圣保罗"),
    ("CDG", "法国巴黎"),
];
/// 查询区域名称
pub fn get_region_name(code: &str) -> Option<&'static str> {
    for &(region_code, region_name) in REGION_MAP {
        if region_code == code {
            return Some(region_name);
        }
    }
    Some("未知") // 如果未找到，返回 “未知”
}
