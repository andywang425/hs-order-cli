use assert_cmd::cargo::*;
use predicates::prelude::*;

#[test]
fn requires_action_group() {
    let mut cmd = cargo_bin_cmd!("hs-order-cli");
    cmd.arg("1234567890123456789")
        .assert()
        .failure()
        .stderr(predicate::str::is_empty().not());
}

#[test]
fn rejects_non_numeric_order_id() {
    let mut cmd = cargo_bin_cmd!("hs-order-cli");
    cmd.arg("abc123")
        .assert()
        .failure()
        .stderr(predicate::str::contains("订单号必须为纯数字"));
}

#[test]
fn mode_requires_pwd() {
    let mut cmd = cargo_bin_cmd!("hs-order-cli");
    cmd.args(["1234567890123456789", "--mode", "标准"])
        .assert()
        .failure();
}

#[test]
fn hero_unknown_name_fails() {
    let mut cmd = cargo_bin_cmd!("hs-order-cli");
    cmd.args(["1234567890123456789", "--hero", "神谕者"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("未知英雄名称"));
}

#[test]
fn hero_mask_out_of_range_fails() {
    let mut cmd = cargo_bin_cmd!("hs-order-cli");
    cmd.args(["1234567890123456789", "--hero", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("英雄掩码超出范围"));
}

#[test]
fn auto_requires_pwd() {
    let mut cmd = cargo_bin_cmd!("hs-order-cli");
    cmd.args(["1234567890123456789", "--auto", "on"])
        .assert()
        .failure();
}

#[test]
fn query_conflicts_with_settings() {
    let mut cmd = cargo_bin_cmd!("hs-order-cli");
    cmd.args(["1234567890123456789", "--query", "--auto", "on"])
        .assert()
        .failure();
}
