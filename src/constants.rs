//! 常量定义模块

use std::time::Duration;

/// API请求地址
pub const API_URL: &str = "http://139.155.71.163:1000/training/hs.php";
/// 主机地址（Host）
pub const HEADER_HOST: &str = "139.155.71.163:1000";
/// 来源地址（Origin）
pub const HEADER_ORIGIN: &str = "http://139.155.71.163:1000";
/// 连接超时时间
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
/// 请求超时时间
pub const TIMEOUT: Duration = Duration::from_secs(5);
/// 常见浏览器 UA 列表
pub const UA_LIST: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0",
];

/// 请求最大重试次数
pub const MAX_RETRIES: u32 = 2;
/// 指数退避的基准毫秒数
pub const RETRY_BASE_MS: u64 = 200;
/// 成功响应代码
pub const SUCCESS_CODE: i32 = 1;

/// 最大显示记录数
pub const MAX_DISPLAY_RECORDS: usize = 10;

/// 基本信息数组大小
pub const BASIC_INFO_SIZE: usize = 10;

// 数据索引常量
/// 金币记录索引
pub const GOLD_RECORDS_INDEX: usize = 10;
/// 经验记录索引
pub const EXP_RECORDS_INDEX: usize = 11;
/// 对战记录索引
pub const BATTLE_RECORDS_INDEX: usize = 12;
/// 最近一次的上号日期索引
pub const DATE_INDEX: usize = 3;
/// 胜利次数索引
pub const WINS_INDEX: usize = 4;
/// 失败次数索引
pub const LOSSES_INDEX: usize = 5;

/// 英雄职业名称
pub const HERO_NAMES: &[&str] = &[
    "战士",
    "萨满祭司",
    "潜行者",
    "圣骑士",
    "猎人",
    "德鲁伊",
    "术士",
    "法师",
    "牧师",
    "恶魔猎手",
    "死亡骑士",
];
/// 最大英雄掩码值
pub const MAX_HERO_MASK: u32 = (1 << HERO_NAMES.len()) - 1;

// 订单状态
/// 已完成订单状态
pub const STATUS_FINISHED: &str = "1";
/// 进行中订单状态
pub const STATUS_RUNNING: &str = "0";
/// 已终止订单状态
pub const STATUS_BANNED: &str = "1";

// 对战模式常量
/// 休闲模式
pub const MODE_CASUAL: &str = "1";
/// 标准模式
pub const MODE_STANDARD: &str = "2";
/// 狂野模式
pub const MODE_WILD: &str = "3";
/// 幻变模式
pub const MODE_TWIST: &str = "4";
/// 酒馆战棋
pub const MODE_BATTLEGROUNDS: &str = "5";

// 对战结果常量
/// 对战胜利
pub const BATTLE_WIN: i64 = 1;
/// 对战失败
pub const BATTLE_LOSS: i64 = -1;
/// 对战结果未知
pub const BATTLE_UNKNOWN: i64 = 0;
