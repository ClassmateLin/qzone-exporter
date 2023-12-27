use thiserror::Error;

#[derive(Error, Debug)]
pub enum QZoneError {
    #[error("解析数据失败!")]
    ParseDataError,

    #[error("网络请求错误, 获取二维码失败!")]
    ReqwestError,

    #[error("解码数据错误, 获取二维码失败!")]
    DecodeError,

    #[error("解析Cookie失败, 获取二维码失败!")]
    GetCookieError,

    #[error("文件读写错误, 保存二维码失败!")]
    FileError,

    #[error("检查二维码扫码状态失败!")]
    CheckQRCodeStatusError,
}

impl serde::Serialize for QZoneError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
