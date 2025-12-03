//! 网络请求模块

use crate::constants::{
    API_URL, CONNECT_TIMEOUT, HEADER_HOST, HEADER_ORIGIN, MAX_RETRIES, RETRY_BASE_MS, SUCCESS_CODE,
    TIMEOUT, UA_LIST,
};
use crate::models::{ApiResponse, OrderData};
use anyhow::Result;
use anyhow::{Context, anyhow, bail};
use once_cell::sync::OnceCell;
use rand::prelude::*;
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE, HOST, HeaderMap,
    HeaderName, HeaderValue, ORIGIN, REFERER,
};
use std::thread::sleep;
use std::time::Duration;

/// 复用的阻塞版 HTTP 客户端（单例）
static CLIENT: OnceCell<reqwest::blocking::Client> = OnceCell::new();

/// 构建 HTTP 客户端
fn build_client() -> Result<reqwest::blocking::Client> {
    let mut headers = HeaderMap::new();
    let x_requested_with = HeaderName::from_static("x-requested-with");

    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(HOST, HeaderValue::from_static(HEADER_HOST));
    headers.insert(ORIGIN, HeaderValue::from_static(HEADER_ORIGIN));
    headers.insert(REFERER, HeaderValue::from_static(API_URL));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
    );
    headers.insert(x_requested_with, HeaderValue::from_static("XMLHttpRequest"));

    let mut rng = rand::rng();
    let ua = *UA_LIST.choose(&mut rng).unwrap();

    let client = reqwest::blocking::Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(TIMEOUT)
        .default_headers(headers)
        .user_agent(ua)
        .build()
        .context("构建 HTTP 客户端失败")?;

    Ok(client)
}

/// 获取全局 HTTP 客户端实例
fn get_client() -> Result<&'static reqwest::blocking::Client> {
    CLIENT.get_or_try_init(build_client)
}

/// 发送表单
fn send_form<T: serde::Serialize>(form: &T) -> Result<ApiResponse> {
    let client = get_client()?;

    for attempt in 0..=MAX_RETRIES {
        let req = client.post(API_URL).form(form);

        match req.send() {
            Ok(resp) => match resp.error_for_status() {
                Ok(ok_resp) => {
                    let bytes = ok_resp.bytes().context("读取响应失败")?;
                    let api_response: ApiResponse =
                        serde_json::from_slice(&bytes).context("解析响应 JSON 失败")?;
                    return Ok(api_response);
                }
                Err(e) => {
                    if attempt < MAX_RETRIES {
                        let delay = RETRY_BASE_MS * (1 << attempt) as u64;
                        sleep(Duration::from_millis(delay));
                        continue;
                    }
                    return Err(anyhow!("请求超过最大重试次数: {}", e));
                }
            },
            Err(e) => {
                if attempt < MAX_RETRIES {
                    let delay = RETRY_BASE_MS * (1 << attempt) as u64;
                    sleep(Duration::from_millis(delay));
                    continue;
                }
                return Err(anyhow!("网络请求失败: {}", e));
            }
        }
    }

    unreachable!();
}

/// 获取订单数据
pub fn fetch_order_data(order_id: &str) -> Result<OrderData> {
    let api_response = send_form(&[("key", order_id)])?;

    if api_response.code != SUCCESS_CODE {
        bail!("API错误({}): {}", api_response.code, api_response.error);
    }

    match api_response.data {
        Some(mut data) if !data.is_empty() => Ok(data.remove(0)),
        _ => bail!("API返回空数据"),
    }
}

/// 设置对战模式
pub fn set_battle_mode(oid: &str, bnetpwd: &str, battlemode: &str) -> Result<()> {
    let api_response = send_form(&[
        ("battlemode", battlemode),
        ("oid", oid),
        ("bnetpwd", bnetpwd),
    ])?;

    if api_response.code != SUCCESS_CODE {
        bail!("API错误({}): {}", api_response.code, api_response.error);
    }

    Ok(())
}

/// 设置对战英雄
pub fn set_battle_heroes(oid: &str, bnetpwd: &str, battleheroes: &str) -> Result<()> {
    let api_response = send_form(&[
        ("battleheroes", battleheroes),
        ("oid", oid),
        ("bnetpwd", bnetpwd),
    ])?;

    if api_response.code != SUCCESS_CODE {
        bail!("API错误({}): {}", api_response.code, api_response.error);
    }

    Ok(())
}

/// 设置是否自动领取奖励
pub fn set_auto_claim(oid: &str, bnetpwd: &str, auto: &str) -> Result<()> {
    let api_response = send_form(&[("auto", auto), ("oid", oid), ("bnetpwd", bnetpwd)])?;

    if api_response.code != SUCCESS_CODE {
        bail!("API错误({}): {}", api_response.code, api_response.error);
    }

    Ok(())
}
