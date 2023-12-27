#[cfg(test)]
mod test {
    use std::{thread::sleep, time::Duration};

    use qzone_exporter::qzone;
    #[tokio::test]
    pub async fn test_get_login_qrcode() {
        let qrcode = qzone::qrcode::get_login_qrcode().await.unwrap();
        for _ in 0..10 {
            let res = qzone::qrcode::get_login_result(qrcode.clone())
                .await
                .unwrap();
            sleep(Duration::from_secs(3));
            if res.msg.contains("成功") {
                break;
            }
            println!("{:?}", res);
        }
    }
}
