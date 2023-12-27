use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::qzone::{client::get_http2_client, error::QZoneError, match_json};

use super::QZoneCookie;

#[derive(Debug, Serialize, Deserialize)]
pub struct QZoneUser {
    uin: u64,
    is_famous: bool,
    nickname: String,
}

#[tauri::command(async)]
pub async fn get_user_info(ck: QZoneCookie) -> Result<QZoneUser, QZoneError> {
    let url = "https://user.qzone.qq.com/proxy/domain/base.qzone.qq.com/cgi-bin/user/cgi_userinfo_get_all";

    let params = json!({
        //"g_tk": "1677593884",
        "callback": "shine0_Callback",
        //"t": "211809104",
        "hostUin": ck.qzone_code,
        "uin": ck.qzone_code,
        "appid": "4",
        "inCharset": "utf-8",
        "outCharset": "utf-8",
        "source": "qzone",
        "plat": "qzone",
        "format": "jsonp",
        "notice": "0",
        "filter": "1",
        "handset": "4",
        "pageNumModeSort": "40",
        "pageNumModeClass": "15",
        "needUserInfo": "1",
        "idcNum": "4",
        "callbackFun": "shine0",
        //"_": "1704433182454"
    });

    let client = get_http2_client().await?;

    let content = client
        .get(url)
        .header("cookie", ck.to_string())
        .query(&params)
        .send()
        .await
        .map_err(|_| QZoneError::ReqwestError)?
        .text()
        .await
        .map_err(|_| QZoneError::ReqwestError)?;

    let data = match_json(content)?;

    let user: QZoneUser = serde_json::from_str(&data).map_err(|_| QZoneError::DecodeError)?;

    Ok(user)
}
