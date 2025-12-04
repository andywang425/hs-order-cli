//! # 亿唐网游专营店 - 炉石传说代练订单助手

mod api;
mod constants;
mod display;
mod models;
mod parser;
mod utils;

use crate::api::fetch_order_data;
use crate::constants::{
    HERO_NAMES, MAX_HERO_MASK, MODE_BATTLEGROUNDS, MODE_CASUAL, MODE_STANDARD, MODE_TWIST,
    MODE_WILD,
};
use crate::display::{display_game_data, display_order_info, print_header};
use crate::parser::{parse_dldata, parse_order_config};
use anyhow::{Context, Result};
use clap::{Args, Parser, ValueEnum};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "hs-order-cli",
    version,
    about = "炉石传说代练订单助手",
    long_about = "亿唐网游专营店 - 炉石传说代练订单助手\n支持订单数据查询和订单相关操作：设置对战模式，设置对战英雄，设置是否自动领取奖励"
)]
struct Cli {
    #[arg(value_name = "ORDER_ID", required = true, help = "订单号", value_parser = parse_order_id)]
    order_id: String,

    #[command(flatten)]
    actions: Actions,

    #[arg(
        short = 't',
        long = "table-size",
        value_name = "NUM",
        default_value_t = 10,
        help = "游戏数据统计表格显示的最大记录条数",
        long_help = "游戏数据统计表格显示的最大记录条数，默认值为 10\n\nALL 表示显示所有记录，0 表示不显示表格",
        value_parser = parse_table_size
    )]
    table_size: usize,

    #[arg(short, long, help = "战网密码前4位", value_parser = parse_pwd4)]
    pwd: Option<String>,

    #[arg(
        short,
        long,
        help = "跳过查询订单数据",
        long_help = "跳过查询订单数据\n\n仅在传入订单相关操作选项时有效，直接将传入的订单号作为订单编号"
    )]
    skip_query: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ModeArg {
    #[value(alias = "1", alias = "c", alias = "休闲")]
    Casual,
    #[value(alias = "2", alias = "s", alias = "标准")]
    Standard,
    #[value(alias = "3", alias = "w", alias = "狂野")]
    Wild,
    #[value(alias = "4", alias = "t", alias = "幻变")]
    Twist,
    #[value(
        alias = "5",
        alias = "b",
        alias = "酒馆",
        alias = "战棋",
        alias = "酒馆战棋"
    )]
    Battlegrounds,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum AutoArg {
    #[value(alias = "1", alias = "true")]
    On,
    #[value(alias = "0", alias = "false")]
    Off,
}

#[derive(Args)]
#[group(required = true, multiple = true)]
struct Actions {
    #[arg(short, long, help = "查询订单数据", long_help = "查询订单数据\n\n不能与订单相关操作选项被同时传入", conflicts_with_all = ["mode", "hero", "auto"])]
    query: bool,

    #[arg(
        short,
        long,
        value_name = "MODE",
        requires = "pwd",
        help = "设置对战模式",
        long_help = "设置对战模式，支持中英文和简称\n\n别名说明:\ncasual = 1|c|休闲\nstandard = 2|s|标准\nwild = 3|w|狂野\ntwist = 4|t|幻变\nbattlegrounds = 5|b|酒馆|战棋|酒馆战棋"
    )]
    mode: Option<ModeArg>,
    #[arg(
        short = 'H',
        long,
        value_name = "HERO",
        value_delimiter = ',',
        requires = "pwd",
        help = "设置对战英雄",
        long_help = "设置对战英雄: 支持英雄名称列表(英文逗号分隔)、掩码数值和全部/ALL\n\n可选英雄: 战士/萨满祭司/潜行者/圣骑士/猎人/德鲁伊/术士/法师/牧师/恶魔猎手/死亡骑士\n掩码计算: 每个英雄按上述顺序由低位到高位依次对应一个二进制位，将允许使用的英雄的对应位设为1，其余位设为0，转为十进制数",
        value_parser = parse_hero_item
    )]
    hero: Option<Vec<String>>,
    #[arg(
        short,
        long,
        value_name = "ON/OFF",
        requires = "pwd",
        help = "设置是否自动领取奖励",
        long_help = "设置是否自动领取奖励\n\n别名说明:\non = 1|true\noff = 0|false"
    )]
    auto: Option<AutoArg>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let order_id = cli.order_id.as_str();
    print_header();

    if cli.actions.query {
        match process_order(order_id, cli.table_size) {
            Ok(_) => {
                println!("{}", "查询完成！".bright_green());
            }
            Err(e) => {
                print_error("查询失败", &e);
            }
        }
        return Ok(());
    }

    let has_settings =
        cli.actions.mode.is_some() || cli.actions.hero.is_some() || cli.actions.auto.is_some();

    if has_settings {
        let oid = match resolve_oid(order_id, cli.skip_query) {
            Ok(o) => o,
            Err(e) => {
                print_error("查询订单编号失败", &e);
                return Ok(());
            }
        };

        println!("订单编号 {}", oid.bright_cyan());

        if let Some(mode_input) = cli.actions.mode
            && let Err(e) = set_battle_mode(&oid, mode_input, cli.pwd.as_deref().unwrap())
        {
            print_error("设置对战模式失败", &e);
        }

        if let Some(hero_inputs) = cli.actions.hero.as_deref()
            && let Err(e) = set_current_hero(&oid, hero_inputs, cli.pwd.as_deref().unwrap())
        {
            print_error("设置对战英雄失败", &e);
        }

        if let Some(auto_input) = cli.actions.auto
            && let Err(e) = set_auto_claim(&oid, auto_input, cli.pwd.as_deref().unwrap())
        {
            print_error("设置自动领取奖励失败", &e);
        }

        return Ok(());
    }

    Ok(())
}

/// 处理单个订单查询
fn process_order(order_id: &str, table_size: usize) -> Result<()> {
    println!("正在查询订单: {}\n", order_id.bright_cyan());

    let order = fetch_order_data(order_id).context("获取订单数据接口失败")?;
    let config = parse_order_config(&order.config).context("解析订单配置信息失败")?;
    let dldata = parse_dldata(&order.dldata).context("解析游戏统计数据失败")?;
    display_order_info(&order, &config, &dldata).context("显示订单基本信息失败")?;
    display_game_data(&dldata, table_size).context("显示游戏数据失败")?;

    Ok(())
}

/// 通过查询订单数据获取订单编号（oid）
fn resolve_oid(order_id: &str, skip_query: bool) -> Result<String> {
    if skip_query {
        return Ok(order_id.to_string());
    }

    let order = fetch_order_data(order_id).context("查询订单数据失败")?;
    Ok(order.oid)
}

/// 设置对战模式
fn set_battle_mode(oid: &str, mode_input: ModeArg, pwd4: &str) -> Result<()> {
    let (normalized, display_name) = match mode_input {
        ModeArg::Casual => (MODE_CASUAL, "休闲模式"),
        ModeArg::Standard => (MODE_STANDARD, "标准模式"),
        ModeArg::Wild => (MODE_WILD, "狂野模式"),
        ModeArg::Twist => (MODE_TWIST, "幻变模式"),
        ModeArg::Battlegrounds => (MODE_BATTLEGROUNDS, "酒馆战棋"),
    };

    api::set_battle_mode(oid, pwd4, normalized).context("设置对战模式接口失败")?;

    println!(
        "{} {}",
        "已设置对战模式为".bright_green(),
        display_name.bright_yellow()
    );

    Ok(())
}

/// 设置对战英雄
fn set_current_hero(oid: &str, hero_inputs: &[String], pwd4: &str) -> Result<()> {
    let mut mask: u32 = 0;

    if hero_inputs.len() == 1 {
        // 输入是全部/ALL或掩码或单个英雄名称
        let lower = hero_inputs[0].to_lowercase();

        if lower == "all" || lower == "全部" {
            mask = MAX_HERO_MASK;
        } else if let Ok(m) = lower.parse::<u32>() {
            // 输入是英雄掩码
            mask = m;
        }
    }

    if mask == 0 {
        // 输入是单个或多个英雄名称（其中可能包含全部/ALL）
        for name in hero_inputs {
            if let Some(idx) = HERO_NAMES.iter().position(|&h| h == name) {
                mask |= 1 << idx;
            } else if name.to_lowercase() == "all" || name == "全部" {
                mask = MAX_HERO_MASK;
            }
        }
    }

    let mut selected_names: Vec<&str> = Vec::new();

    for (i, &name) in HERO_NAMES.iter().enumerate() {
        if mask & (1 << i) != 0 {
            selected_names.push(name);
        }
    }

    api::set_battle_heroes(oid, pwd4, &mask.to_string()).context("设置对战英雄接口失败")?;

    if mask == MAX_HERO_MASK {
        println!(
            "{} {}",
            "已设置对战英雄为".bright_green(),
            "全部".bright_yellow()
        );
    } else {
        println!(
            "{} {}",
            "已设置对战英雄为".bright_green(),
            selected_names.join(", ").bright_yellow()
        );
    }

    Ok(())
}

/// 设置自动领取奖励
fn set_auto_claim(oid: &str, auto_input: AutoArg, pwd4: &str) -> Result<()> {
    let val = match auto_input {
        AutoArg::On => "1",
        AutoArg::Off => "0",
    };

    api::set_auto_claim(oid, pwd4, val).context("设置自动领取接口失败")?;

    println!(
        "{} {}",
        "已设置自动领取奖励为".bright_green(),
        if val == "1" { "开启" } else { "关闭" }.bright_yellow()
    );

    Ok(())
}

/// 解析订单号
fn parse_order_id(s: &str) -> std::result::Result<String, String> {
    if s.chars().all(|c| c.is_ascii_digit()) {
        Ok(s.to_string())
    } else {
        Err("订单号必须为纯数字".to_string())
    }
}

/// 解析游戏数据统计表格显示的最大记录条数
fn parse_table_size(s: &str) -> std::result::Result<usize, String> {
    if s.to_lowercase() == "all" {
        Ok(usize::MAX)
    } else if let Ok(num) = s.parse::<usize>() {
        Ok(num)
    } else {
        Err("游戏数据统计表格显示的最大记录条数必须为整数或ALL".to_string())
    }
}

/// 解析战网密码前4位
fn parse_pwd4(s: &str) -> std::result::Result<String, String> {
    if s.len() == 4 {
        Ok(s.to_string())
    } else {
        Err("战网密码前4位必须为4个字符".to_string())
    }
}

/// 解析对战英雄输入项
fn parse_hero_item(s: &str) -> std::result::Result<String, String> {
    let lower = s.to_lowercase();

    if lower == "all" || s == "全部" {
        return Ok(s.to_string());
    }

    if let Ok(m) = s.parse::<u32>() {
        if m > MAX_HERO_MASK || m == 0 {
            return Err(format!("英雄掩码超出范围(1-{}): {}", MAX_HERO_MASK, m));
        }

        return Ok(s.to_string());
    }

    if HERO_NAMES.contains(&s) {
        Ok(s.to_string())
    } else {
        Err(format!("未知英雄名称: {}", s))
    }
}

/// 打印错误信息
fn print_error(context: &str, e: &anyhow::Error) {
    print!("{}", context.bright_red());

    for cause in e.chain() {
        print!(" {}", cause);
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_order_id_ok() {
        let s = "1234567890123456789";
        let r = parse_order_id(s);
        assert_eq!(r.unwrap(), s);
    }

    #[test]
    fn test_parse_order_id_err() {
        let r = parse_order_id("abc123");
        assert!(matches!(r, Err(e) if e == "订单号必须为纯数字"));
    }

    #[test]
    fn test_parse_table_size_ok() {
        let s = "30";
        let r = parse_table_size(s);
        assert_eq!(r.unwrap(), 30);
    }

    #[test]
    fn test_parse_table_size_all() {
        let r = parse_table_size("all");
        assert_eq!(r.unwrap(), usize::MAX);
    }

    #[test]
    fn test_parse_table_size_err() {
        let r = parse_table_size("abc");
        assert!(matches!(r, Err(e) if e == "游戏数据统计表格显示的最大记录条数必须为整数或ALL"));
    }

    #[test]
    fn test_parse_pwd4_ok() {
        let s = "zwmm";
        let r = parse_pwd4(s);
        assert_eq!(r.unwrap(), s);
    }

    #[test]
    fn test_parse_pwd4_err() {
        let r = parse_pwd4("zw3");
        assert!(matches!(r, Err(e) if e == "战网密码前4位必须为4个字符"));
    }

    #[test]
    fn test_parse_hero_item_all() {
        assert_eq!(parse_hero_item("all").unwrap(), "all");
        assert_eq!(parse_hero_item("全部").unwrap(), "全部");
    }

    #[test]
    fn test_parse_hero_item_mask_ok() {
        let r = parse_hero_item("1234");
        assert_eq!(r.unwrap(), "1234");
    }

    #[test]
    fn test_parse_hero_item_mask_err_low() {
        let r = parse_hero_item("0");
        let expected = format!("英雄掩码超出范围(1-{}): {}", MAX_HERO_MASK, 0);
        assert!(matches!(r, Err(e) if e == expected));
    }

    #[test]
    fn test_parse_hero_item_mask_err_high() {
        let high = (MAX_HERO_MASK + 1).to_string();
        let expected = format!(
            "英雄掩码超出范围(1-{}): {}",
            MAX_HERO_MASK,
            MAX_HERO_MASK + 1
        );
        let r = parse_hero_item(&high);
        assert!(matches!(r, Err(e) if e == expected));
    }

    #[test]
    fn test_parse_hero_item_name_ok() {
        let name = HERO_NAMES[7];
        let r = parse_hero_item(name);
        assert_eq!(r.unwrap(), name);
    }

    #[test]
    fn test_parse_hero_item_name_err() {
        let r = parse_hero_item("神谕者");
        assert!(matches!(r, Err(e) if e == "未知英雄名称: 神谕者"));
    }
}
