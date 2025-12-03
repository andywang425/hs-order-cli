//! 工具函数模块

use chrono::{Local, TimeZone};

/// 格式化时间戳为可读格式
pub fn format_timestamp(timestamp: i64) -> String {
    if timestamp == 0 {
        return "未知时间".to_string();
    }

    match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        _ => "时间格式错误".to_string(),
    }
}

/// 将数字格式化为带符号的字符串
pub fn format_signed<T: std::fmt::Display>(num: T) -> String {
    let s = format!("{:+}", num);
    if s == "+0" { "0".to_string() } else { s }
}

/// 解析无符号整数字符串
///
/// 如果是大于等于0的整数，正常解析；否则解析为0
pub fn parse_unsigned_int(s: &str) -> usize {
    s.parse().unwrap_or(0).max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_timestamp_zero() {
        assert_eq!(format_timestamp(0), "未知时间");
    }

    #[test]
    fn test_format_timestamp_invalid() {
        assert_eq!(format_timestamp(i64::MAX), "时间格式错误");
    }

    #[test]
    fn test_format_signed_number() {
        assert_eq!(format_signed(3usize), "+3");
        assert_eq!(format_signed(3i64), "+3");
        assert_eq!(format_signed(-3i64), "-3");
    }

    #[test]
    fn test_format_signed_number_zero() {
        assert_eq!(format_signed(0usize), "0");
        assert_eq!(format_signed(0i64), "0");
    }

    #[test]
    fn test_parse_signed_number_positive() {
        assert_eq!(parse_unsigned_int("+5"), 5);
    }

    #[test]
    fn test_parse_signed_number_negative() {
        assert_eq!(parse_unsigned_int("-3"), 0);
    }

    #[test]
    fn test_parse_signed_number_invalid() {
        assert_eq!(parse_unsigned_int("abc"), 0);
    }
}
