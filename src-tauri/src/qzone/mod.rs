use regex::Regex;
use serde::{Deserialize, Serialize};

use self::error::QZoneError;
pub mod client;
pub mod error;
pub mod qrcode;
pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct QZoneCookie {
    #[serde(rename = "RK")]
    pub rk: String,
    pub p_skey: String,
    pub p_uin: String,
    pub pt2gguin: String,
    pub pt4_token: String,
    pub pt_recent_uins: String,
    pub ptcz: String,
    pub superkey: String,
    pub supertoken: String,
    pub superuin: String,
    pub uin: String,
    pub qzone_code: String,
}

impl ToString for QZoneCookie {
    fn to_string(&self) -> String {
        format!("RK={};p_skey={};p_uin={};pt2gguin={};pt4_token={};pt_recent_uins={};ptcz={};superkey={};supertoken={};superuin={};uin={};",
            self.rk, self.p_skey, self.p_uin, self.pt2gguin, self.pt4_token, self.pt_recent_uins, self.ptcz, self.superkey, self.supertoken, self.superuin,self.uin
        )
    }
}

pub fn match_json(content: String) -> Result<String, QZoneError> {
    let re = Regex::new(r#"_Callback\((\{.*\})\);"#).unwrap();
    if let Some(captures) = re.captures(&content.replace('\n', "")) {
        let json_content = captures.get(1).unwrap().as_str();
        return Ok(json_content.into());
    }
    Err(QZoneError::ParseDataError)
}
