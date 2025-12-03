//! 数据解析模块

use crate::constants::*;
use crate::models::{BattleRecord, DlData, ExpRecord, GoldRecord, OrderConfig};
use crate::utils::{format_signed, format_timestamp};
use anyhow::{Context, Result, anyhow, bail};
use chrono::Local;
use chrono_tz::Asia::Shanghai;

/// 解析config字段（订单配置信息）
pub fn parse_order_config(config_str: &str) -> Result<OrderConfig> {
    let cfg: OrderConfig = serde_json::from_str(config_str)?;
    Ok(cfg)
}

/// 解析dldata字段（游戏统计数据）
pub fn parse_dldata(dldata_str: &str) -> Result<DlData> {
    let arr: Vec<serde_json::Value> =
        serde_json::from_str(dldata_str).context("dldata格式不正确")?;

    if arr.len() < 13 {
        bail!("dldata数据不完整");
    }

    let basic_info = arr.get(0..BASIC_INFO_SIZE).unwrap_or(&[]).to_vec();
    let today_battles = calculate_today_battles(&arr);
    let gold_records = parse_gold_records(&arr)?;
    let exp_records = parse_exp_records(&arr)?;
    let battle_records = parse_battle_records(&arr)?;

    Ok(DlData {
        basic_info,
        gold_records,
        exp_records,
        battle_records,
        today_battles,
    })
}

/// 计算今日对战次数
fn calculate_today_battles(arr: &[serde_json::Value]) -> usize {
    let today = Local::now()
        .with_timezone(&Shanghai)
        .format("%Y%m%d")
        .to_string();

    let date_str = arr.get(DATE_INDEX).and_then(|v| {
        v.as_str()
            .map(|s| s.to_string())
            .or_else(|| v.as_i64().map(|n| n.to_string()))
    });

    if date_str.as_deref() != Some(&today) {
        return 0;
    }

    let wins = arr.get(WINS_INDEX).and_then(|v| v.as_i64()).unwrap_or(0) as usize;
    let losses = arr.get(LOSSES_INDEX).and_then(|v| v.as_i64()).unwrap_or(0) as usize;
    wins + losses
}

/// 解析金币记录
fn parse_gold_records(arr: &[serde_json::Value]) -> Result<Vec<GoldRecord>> {
    let gold_data = arr
        .get(GOLD_RECORDS_INDEX)
        .ok_or_else(|| anyhow!("金币记录索引超出范围"))?;

    if let Some(records_array) = gold_data.as_array() {
        Ok(records_array
            .iter()
            .filter_map(parse_single_gold_record)
            .collect())
    } else {
        Ok(vec![])
    }
}

/// 解析单条金币记录
fn parse_single_gold_record(record: &serde_json::Value) -> Option<GoldRecord> {
    let record_array = record.as_array()?;
    if record_array.len() >= 3 {
        Some(GoldRecord {
            time: format_timestamp(record_array[0].as_i64().unwrap_or(0)),
            gold_change: record_array[1]
                .as_i64()
                .map(format_signed)
                .unwrap_or_else(|| "0".to_string()),
            pack_change: record_array[2]
                .as_i64()
                .map(format_signed)
                .unwrap_or_else(|| "0".to_string()),
        })
    } else {
        None
    }
}

/// 解析经验记录
fn parse_exp_records(arr: &[serde_json::Value]) -> Result<Vec<ExpRecord>> {
    let exp_data = arr
        .get(EXP_RECORDS_INDEX)
        .ok_or_else(|| anyhow!("经验记录索引超出范围"))?;

    if let Some(records_array) = exp_data.as_array() {
        Ok(records_array
            .iter()
            .filter_map(parse_single_exp_record)
            .collect())
    } else {
        Ok(vec![])
    }
}

/// 解析单条经验记录
fn parse_single_exp_record(record: &serde_json::Value) -> Option<ExpRecord> {
    let record_array = record.as_array()?;
    if record_array.len() >= 5 {
        Some(ExpRecord {
            time: format_timestamp(record_array[0].as_i64().unwrap_or(0)),
            exp_change: record_array[1]
                .as_i64()
                .map(format_signed)
                .unwrap_or_else(|| "0".to_string()),
            level: record_array[2]
                .as_i64()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "0".to_string()),
            total_exp: record_array[3]
                .as_i64()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "0".to_string()),
            current_level_exp: record_array[4]
                .as_i64()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "0".to_string()),
        })
    } else {
        None
    }
}

/// 解析对战记录
fn parse_battle_records(arr: &[serde_json::Value]) -> Result<Vec<BattleRecord>> {
    let battle_data = arr
        .get(BATTLE_RECORDS_INDEX)
        .ok_or_else(|| anyhow!("对战记录索引超出范围"))?;

    if let Some(records_array) = battle_data.as_array() {
        Ok(records_array
            .iter()
            .filter_map(parse_single_battle_record)
            .collect())
    } else {
        Ok(vec![])
    }
}

/// 解析单条对战记录
fn parse_single_battle_record(record: &serde_json::Value) -> Option<BattleRecord> {
    let record_array = record.as_array()?;
    if record_array.len() >= 4 {
        let start_timestamp = record_array[3].as_i64().unwrap_or(0);
        let result_code = record_array[1].as_i64().unwrap_or(0);

        Some(BattleRecord {
            time: format_timestamp(start_timestamp),
            result: get_battle_result_text(result_code),
            exp: record_array[2]
                .as_i64()
                .map(format_signed)
                .unwrap_or_else(|| "0".to_string()),
        })
    } else {
        None
    }
}

/// 获取对战结果文本
fn get_battle_result_text(result_code: i64) -> String {
    match result_code {
        BATTLE_WIN => "胜利".into(),
        BATTLE_LOSS => "失败".into(),
        BATTLE_UNKNOWN => "未知".into(),
        _ => format!("未知 {}", result_code),
    }
}

/// 解析对战英雄掩码
pub fn parse_battle_heroes(battleheroes: &str) -> Vec<String> {
    let mask = battleheroes.parse::<u32>().unwrap_or(0);
    let mut heroes = Vec::new();

    for (i, &hero_name) in HERO_NAMES.iter().enumerate() {
        if mask & (1 << i) != 0 {
            heroes.push(hero_name.to_string());
        }
    }

    heroes
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_order_config_ok() {
        let cfg = json!({
            "battlemode": "3",
            "region": "CN",
            "battleheroes": "2047",
        });
        let s = cfg.to_string();
        let r = parse_order_config(&s).unwrap();
        assert_eq!(r.battlemode.as_deref(), Some("3"));
        assert_eq!(r.battleheroes.as_deref(), Some("2047"));
        assert_eq!(r.auto.as_deref(), None);
    }

    #[test]
    fn test_parse_dldata_ok() {
        let now = chrono::Local::now().with_timezone(&Shanghai);
        let today = now.format("%Y%m%d").to_string();
        let timestamp = now.timestamp().to_string();
        let mut arr = vec![json!(null); 13];
        arr[0] = json!(8560);
        arr[1] = json!(9);
        arr[2] = json!(50);
        arr[3] = json!(today);
        arr[4] = json!(3);
        arr[5] = json!(27);
        arr[6] = json!(0);
        arr[7] = json!("CHN");
        arr[8] = json!(timestamp);
        arr[9] = json!(0);
        arr[10] = json!([
            [1759898700, 50, 0],
            [1759890783, 0, 2],
            [1759887285, 100, 2],
            [1759885254, 50, 1],
        ]);
        arr[11] = json!([
            [1762920243, 139, 43, 30693, 93],
            [1762918710, 127, 42, 30554, 1454],
            [1762917298, 112, 42, 30427, 1327],
        ]);
        arr[12] = json!([
            [1762928742, -1, 14, 1762927893],
            [0, 0, 0, 1762925746],
            [1762925584, 1, 154, 1762924010],
        ]);
        let s = serde_json::to_string(&arr).unwrap();
        let d = parse_dldata(&s).unwrap();
        assert_eq!(d.today_battles, 30);
        assert_eq!(d.gold_records.len(), 4);
        assert_eq!(d.gold_records[0].time, "2025-10-08 12:45:00");
        assert_eq!(d.gold_records[0].gold_change, "+50");
        assert_eq!(d.gold_records[0].pack_change, "0");
        assert_eq!(d.exp_records.len(), 3);
        assert_eq!(d.exp_records[0].time, "2025-11-12 12:04:03");
        assert_eq!(d.exp_records[0].exp_change, "+139");
        assert_eq!(d.exp_records[0].level, "43");
        assert_eq!(d.exp_records[0].total_exp, "30693");
        assert_eq!(d.exp_records[0].current_level_exp, "93");
        assert_eq!(d.battle_records.len(), 3);
        assert_eq!(d.battle_records[0].time, "2025-11-12 14:11:33");
        assert!(d.battle_records[0].result.contains("失败"));
        assert_eq!(d.battle_records[0].exp, "+14");
        assert!(d.battle_records[1].result.contains("未知"));
        assert_eq!(d.battle_records[1].exp, "0");
    }

    #[test]
    fn test_parse_battle_heroes_mask() {
        let v = parse_battle_heroes("2000");
        assert!(v.len() == 6);
        assert_eq!(v[0], "猎人");
        assert_eq!(v[1], "术士");
        assert_eq!(v[2], "法师");
        assert_eq!(v[3], "牧师");
        assert_eq!(v[4], "恶魔猎手");
        assert_eq!(v[5], "死亡骑士");
    }
}
