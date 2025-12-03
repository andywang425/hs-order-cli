//! 数据模型模块

use serde::Deserialize;
use tabled::Tabled;

/// API响应顶层结构
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub code: i32,
    pub error: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub count: Option<i32>,
    pub data: Option<Vec<OrderData>>,
}

/// 订单基本信息
#[derive(Debug, Deserialize)]
pub struct OrderData {
    #[allow(dead_code)]
    pub am: String,
    pub oid: String,
    pub edate: String,
    pub config: String,
    #[allow(dead_code)]
    pub details: String,
    pub finish: String,
    pub banned: String,
    #[allow(dead_code)]
    pub dltype: String,
    pub num1: String,
    pub num2: String,
    pub num3: String,
    #[allow(dead_code)]
    pub num7: String,
    #[allow(dead_code)]
    pub num8: String,
    pub dldata: String,
    pub remark: String,
}

/// 订单配置信息
#[derive(Debug, Deserialize, Default)]
pub struct OrderConfig {
    pub battlemode: Option<String>,
    #[allow(dead_code)]
    pub region: Option<String>,
    #[allow(dead_code)]
    pub pause: Option<String>,
    pub battleheroes: Option<String>,
    pub auto: Option<String>,
}

/// 游戏数据统计信息
#[derive(Debug)]
pub struct DlData {
    #[allow(dead_code)]
    pub basic_info: Vec<serde_json::Value>,
    pub gold_records: Vec<GoldRecord>,
    pub exp_records: Vec<ExpRecord>,
    pub battle_records: Vec<BattleRecord>,
    pub today_battles: usize,
}

/// 金币记录
#[derive(Debug, Tabled)]
pub struct GoldRecord {
    #[tabled(rename = "时间")]
    pub time: String,
    #[tabled(rename = "金币变化")]
    pub gold_change: String,
    #[tabled(rename = "卡包变化")]
    pub pack_change: String,
}

/// 经验记录
#[derive(Debug, Tabled)]
pub struct ExpRecord {
    #[tabled(rename = "时间")]
    pub time: String,
    #[tabled(rename = "经验变化")]
    pub exp_change: String,
    #[tabled(rename = "等级")]
    pub level: String,
    #[tabled(rename = "总经验")]
    pub total_exp: String,
    #[tabled(rename = "当前等级经验")]
    pub current_level_exp: String,
}

/// 对战记录
#[derive(Debug, Tabled)]
pub struct BattleRecord {
    #[tabled(rename = "时间")]
    pub time: String,
    #[tabled(rename = "结果")]
    pub result: String,
    #[tabled(rename = "经验")]
    pub exp: String,
}
