use std::path::PathBuf;

use super::{client::get_http_client, error::QZoneError, QZoneCookie};
use anyhow::{Error, Result};
use chrono::Local;
use num_bigint::BigUint;
use regex::Regex;
use reqwest::header::{self, HeaderValue, COOKIE, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::{fs::File, io::AsyncWriteExt};

const QRCODE_VALID: &str = "二维码未失效";
const QRCODE_EXPIRED: &str = "二维码已失效";
const QRCODE_VERIFYING: &str = "二维码认证中";
const QRCODE_SUCCESS: &str = "二维码认证成功";
const QRCODE_UNKNOWN: &str = "二维码状态未知";

#[derive(Debug, Serialize, Deserialize)]
pub enum QRCodeResultCode {
    Valid,     // 二维码未失效
    Expired,   // 二维码已失效
    VERIFYING, // 二维码认证中
    Success,   // 二维码认证成功
    Unknown,   // 二维码状态未知
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QRCodeLoginResult {
    pub code: QRCodeResultCode,
    pub msg: String,
    pub data: Option<QZoneCookie>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QRCode {
    pub qrcode_path: String, // 二维码图片保存路径
    pub qrsig: String,       // 二维码签名
    pub ptqrtoken: String,   // 二维码token
}

// 计算ptqrtoken
fn get_ptqrtoken(qrsig: &str) -> String {
    let mut e: BigUint = BigUint::from(0_u32);
    for c in qrsig.chars() {
        e += (e.clone() << 5) + c as u32;
    }
    let i = BigUint::from(2147483647_u32);
    (i & e).to_string()
}

fn get_base_dir() -> Result<PathBuf, Error> {
    let path = match project_root::get_project_root() {
        Ok(p) => Ok(p),
        Err(e) => Err(e),
    }?;

    Ok(path.parent().unwrap().into())
}

// 获取QQ空间登录二维码
#[tauri::command(async)]
pub async fn get_login_qrcode() -> Result<QRCode, QZoneError> {
    let url = "https://ssl.ptlogin2.qq.com/ptqrshow?appid=549000912&e=2&l=M&s=3&d=72&v=4&t=0.8692955245720428&daid=5&pt_3rd_aid=0";

    let client = get_http_client().await?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|_| QZoneError::ReqwestError)?;

    let mut qrsig = String::new();

    for v in response.cookies() {
        if v.name().starts_with("qrsig") {
            qrsig = v.value().to_string();
        }
    }

    if qrsig.is_empty() {
        return Err(QZoneError::GetCookieError);
    }

    let ptqrtoken = get_ptqrtoken(qrsig.as_str());

    let body = response
        .bytes()
        .await
        .map_err(|_| QZoneError::DecodeError)?;

    let qrcode_path = String::from("public/imgs/.qrcode.png");
    let abs_qrcode_path = get_base_dir()
        .map_err(|_| QZoneError::FileError)?
        .join(&qrcode_path);

    let mut file = File::create(&abs_qrcode_path)
        .await
        .map_err(|_| QZoneError::FileError)?;
    file.write_all(&body)
        .await
        .map_err(|_| QZoneError::FileError)?;

    Ok(QRCode {
        qrcode_path,
        qrsig,
        ptqrtoken,
    })
}

// 检查二维码状态
#[tauri::command(async)]
pub async fn get_login_result(qrcode: QRCode) -> Result<QRCodeLoginResult, QZoneError> {
    let cookie = HeaderValue::from_str(&format!("qrsig={}", qrcode.qrsig))
        .map_err(|_| QZoneError::CheckQRCodeStatusError)?;

    let mut headers = header::HeaderMap::new();
    headers.insert(USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"));
    headers.insert(COOKIE, cookie);

    let cookies_store = reqwest_cookie_store::CookieStore::new(None);
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookies_store);
    let cookie_store = std::sync::Arc::new(cookie_store);

    let url = format!("https://ssl.ptlogin2.qq.com/ptqrlogin?u1=https%3A%2F%2Fqzs.qq.com%2Fqzone%2Fv5%2Floginsucc.html%3Fpara%3Dizone&ptqrtoken={}&ptredirect=0&h=1&t=1&g=1&from_ui=1&ptlang=2052&action=0-0-{}&js_ver=20032614&js_type=1&login_sig=&pt_uistyle=40&aid=549000912&daid=5&", qrcode.ptqrtoken, Local::now().timestamp_millis());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .https_only(true)
        .cookie_provider(cookie_store.clone())
        .build()
        .map_err(|_| QZoneError::CheckQRCodeStatusError)?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|_| QZoneError::CheckQRCodeStatusError)?;

    let content = response
        .text()
        .await
        .map_err(|_| QZoneError::CheckQRCodeStatusError)?;

    if content.contains("二维码未失效") {
        Ok(QRCodeLoginResult {
            code: QRCodeResultCode::Valid,
            msg: QRCODE_VALID.to_string(),
            data: None,
        })
    } else if content.contains("二维码认证中") {
        Ok(QRCodeLoginResult {
            code: QRCodeResultCode::VERIFYING,
            msg: QRCODE_VERIFYING.to_string(),
            data: None,
        })
    } else if content.contains("二维码已失效") {
        Ok(QRCodeLoginResult {
            code: QRCodeResultCode::Expired,
            msg: QRCODE_EXPIRED.to_string(),
            data: None,
        })
    } else if content.contains("登录成功") {
        let regex =
            Regex::new(r"(?P<url>https?://[-A-Za-z0-9+&@#/%?=~_|!:,.;]+[-A-Za-z0-9+&@#/%=~_|])")
                .map_err(|_| QZoneError::CheckQRCodeStatusError)?;
        let url = match regex.captures(&content) {
            Some(cap) => cap.name("url").unwrap().as_str(),
            None => "",
        };
        if url.is_empty() {
            return Ok(QRCodeLoginResult {
                code: QRCodeResultCode::Unknown,
                msg: QRCODE_UNKNOWN.to_string(),
                data: None,
            });
        }
        let _ = client
            .get(url)
            .send()
            .await
            .map_err(|_| QZoneError::CheckQRCodeStatusError)?;
        let store = cookie_store.lock().unwrap();
        let mut ck = json!({});
        for item in store.iter_any() {
            if item.name().starts_with("skey") {
                continue;
            }
            if item.name().starts_with("uin") {
                ck["qzone_code"] = item.value().replace('o', "").into();
            }
            if item.value().is_empty() {
                continue;
            }
            ck[item.name()] = item.value().into();
        }
        let ck: QZoneCookie = serde_json::from_value(ck).map_err(|_| QZoneError::GetCookieError)?;

        Ok(QRCodeLoginResult {
            code: QRCodeResultCode::Success,
            msg: QRCODE_SUCCESS.to_string(),
            data: Some(ck),
        })
    } else {
        Ok(QRCodeLoginResult {
            code: QRCodeResultCode::Unknown,
            msg: QRCODE_UNKNOWN.to_string(),
            data: None,
        })
    }
}
