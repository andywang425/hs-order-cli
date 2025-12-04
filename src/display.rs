//! 显示模块

use crate::constants::*;
use crate::models::{BattleRecord, DlData, ExpRecord, GoldRecord, OrderConfig, OrderData};
use crate::parser::parse_battle_heroes;
use crate::utils::{format_signed, parse_unsigned_int};
use anyhow::Result;
use colored::*;
use tabled::{Table, Tabled};

/// 打印程序标题
pub fn print_header() {
    println!("{}", "=== 炉石传说代练订单助手 ===".bright_cyan().bold());
    println!();
}

/// 打印分隔线
pub fn print_line() {
    println!("{}", "━".repeat(50));
}

/// 显示订单基本信息
pub fn display_order_info(order: &OrderData, config: &OrderConfig, dldata: &DlData) -> Result<()> {
    println!("{}", "订单基本信息".bright_blue().bold());
    print_line();

    println!("订单编号: {}", order.oid.bright_cyan());
    println!("截止时间: {}", order.edate.bright_white());
    println!(
        "订单状态: {}",
        get_order_status(&order.finish, &order.banned)
    );
    println!(
        "对战模式: {}",
        get_battle_mode_text(config.battlemode.as_deref().unwrap_or(MODE_CASUAL))
    );
    println!("对战英雄: {}", get_battle_heroes_text(&config.battleheroes));
    println!("自动领取: {}", get_auto_claim_text(&config.auto));
    println!("金币数量: {} 枚", order.num1.bright_yellow());
    println!("卡包数量: {} 包", order.num2.bright_blue());
    let reward_level = order.num3.parse::<i64>().unwrap_or(0) + 1;
    println!("奖励等级: {} 级", reward_level.to_string().bright_purple());
    println!(
        "今日对战: {} 场",
        dldata.today_battles.to_string().bright_cyan()
    );

    if !order.remark.is_empty() {
        println!("备注信息: {}", order.remark.bright_white());
    }

    println!();
    Ok(())
}

/// 获取订单状态文本
fn get_order_status(finish: &str, banned: &str) -> ColoredString {
    match (finish, banned) {
        (STATUS_FINISHED, STATUS_RUNNING) => "已完成".bright_green(),
        (STATUS_RUNNING, STATUS_RUNNING) => "进行中".bright_blue(),
        (_, STATUS_BANNED) => "已终止".bright_red(),
        _ => "未知状态".bright_magenta(),
    }
}

/// 获取对战模式文本
fn get_battle_mode_text(battlemode: &str) -> ColoredString {
    match battlemode {
        MODE_CASUAL => "休闲模式".bright_yellow(),
        MODE_STANDARD => "标准模式".bright_yellow(),
        MODE_WILD => "狂野模式".bright_yellow(),
        MODE_TWIST => "幻变模式".bright_yellow(),
        MODE_BATTLEGROUNDS => "酒馆战棋".bright_yellow(),
        _ => format!("未知模式({})", battlemode).bright_magenta(),
    }
}

/// 获取对战英雄文本
fn get_battle_heroes_text(battleheroes: &Option<String>) -> ColoredString {
    match battleheroes {
        Some(heroes_str) => {
            if heroes_str.is_empty() {
                return "无".bright_yellow();
            }

            let heroes = parse_battle_heroes(heroes_str);

            if heroes.is_empty() {
                "无".bright_yellow()
            } else if heroes.len() == HERO_NAMES.len() {
                "全部".bright_blue()
            } else {
                heroes.join(", ").bright_white()
            }
        }
        None => "全部".bright_blue(),
    }
}

/// 获取自动领取文本
fn get_auto_claim_text(auto: &Option<String>) -> ColoredString {
    match auto.as_deref() {
        Some("1") => "开启".bright_green(),
        Some("0") => "关闭".bright_yellow(),
        None => "关闭".bright_yellow(),
        Some(_) => "未知".bright_magenta(),
    }
}

/// 显示游戏数据统计
pub fn display_game_data(dl_data: &DlData, table_size: usize) -> Result<()> {
    display_gold_statistics(&dl_data.gold_records, table_size);
    display_exp_statistics(&dl_data.exp_records, table_size);
    display_battle_statistics(&dl_data.battle_records, table_size);
    Ok(())
}

/// 显示金币统计
fn display_gold_statistics(gold_records: &[GoldRecord], table_size: usize) {
    println!("{}", "金币统计".bright_yellow().bold());
    print_line();

    if gold_records.is_empty() {
        println!("暂无金币记录");
        println!();
        return;
    }

    let (total_gold, total_packs) = calculate_gold_totals(gold_records);

    println!(
        "总金币变化: {} 枚",
        format_signed(total_gold).bright_green()
    );
    println!(
        "总卡包变化: {} 个",
        format_signed(total_packs).bright_blue()
    );
    println!();

    display_records_table(gold_records, "金币记录", table_size);
}

/// 显示经验统计
fn display_exp_statistics(exp_records: &[ExpRecord], table_size: usize) {
    println!("{}", "经验统计".bright_purple().bold());
    print_line();

    if exp_records.is_empty() {
        println!("暂无经验记录");
        println!();
        return;
    }

    let total_exp = calculate_exp_total(exp_records);
    println!(
        "总经验变化: {} 点",
        format_signed(total_exp).bright_purple()
    );
    println!();

    display_records_table(exp_records, "经验记录", table_size);
}

/// 显示对战统计
fn display_battle_statistics(battle_records: &[BattleRecord], table_size: usize) {
    println!("{}", "对战统计".bright_red().bold());
    print_line();

    if battle_records.is_empty() {
        println!("暂无对战记录");
        println!();
        return;
    }

    let (wins, losses, total_exp) = calculate_battle_stats(battle_records);
    let total_battles = wins + losses;
    let win_rate = if total_battles > 0 {
        (wins as f64 / total_battles as f64 * 100.0) as u8
    } else {
        0
    };

    println!("胜利场次: {} 场", wins.to_string().bright_green());
    println!("失败场次: {} 场", losses.to_string().bright_red());
    println!("胜率: {} %", win_rate.to_string().bright_cyan());
    println!("对战经验: {} 点", format_signed(total_exp).bright_purple());
    println!();

    display_records_table(battle_records, "对战记录", table_size);
}

/// 计算金币总计
fn calculate_gold_totals(gold_records: &[GoldRecord]) -> (usize, usize) {
    let mut total_gold = 0;
    let mut total_packs = 0;

    for record in gold_records {
        total_gold += parse_unsigned_int(&record.gold_change);
        total_packs += parse_unsigned_int(&record.pack_change);
    }

    (total_gold, total_packs)
}

/// 计算经验总计
fn calculate_exp_total(exp_records: &[ExpRecord]) -> usize {
    exp_records
        .iter()
        .map(|record| parse_unsigned_int(&record.exp_change))
        .sum()
}

/// 计算对战统计
fn calculate_battle_stats(battle_records: &[BattleRecord]) -> (usize, usize, usize) {
    let mut wins = 0;
    let mut losses = 0;
    let mut total_exp = 0;

    for record in battle_records {
        if record.result.contains("胜利") {
            wins += 1;
        } else if record.result.contains("失败") {
            losses += 1;
        }
        total_exp += parse_unsigned_int(&record.exp);
    }

    (wins, losses, total_exp)
}

/// 显示记录表格
fn display_records_table<T: Tabled>(records: &[T], record_type: &str, table_size: usize) {
    if records.is_empty() || table_size == 0 {
        return;
    }

    let display_records = if records.len() > table_size {
        &records[..table_size]
    } else {
        records
    };

    println!(
        "最近 {} 条{}:",
        display_records.len().to_string().bright_white(),
        record_type
    );
    let table = Table::new(display_records);
    println!("{}", table);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_gold_totals() {
        let items = vec![
            GoldRecord {
                time: String::new(),
                gold_change: "+50".into(),
                pack_change: "0".into(),
            },
            GoldRecord {
                time: String::new(),
                gold_change: "+100".into(),
                pack_change: "+1".into(),
            },
        ];
        let (g, p) = calculate_gold_totals(&items);
        assert_eq!(g, 150);
        assert_eq!(p, 1);
    }

    #[test]
    fn test_calculate_exp_total() {
        let items = vec![
            ExpRecord {
                time: String::new(),
                exp_change: "+161".into(),
                level: String::new(),
                total_exp: String::new(),
                current_level_exp: String::new(),
            },
            ExpRecord {
                time: String::new(),
                exp_change: "+78".into(),
                level: String::new(),
                total_exp: String::new(),
                current_level_exp: String::new(),
            },
        ];
        assert_eq!(calculate_exp_total(&items), 239);
    }

    #[test]
    fn test_calculate_battle_stats() {
        let items = vec![
            BattleRecord {
                time: String::new(),
                result: "胜利".into(),
                exp: "+156".into(),
            },
            BattleRecord {
                time: String::new(),
                result: "失败".into(),
                exp: "+43".into(),
            },
        ];
        let (w, l, e) = calculate_battle_stats(&items);
        assert_eq!(w, 1);
        assert_eq!(l, 1);
        assert_eq!(e, 199);
    }
}
