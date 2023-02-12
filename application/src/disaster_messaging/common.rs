use super::request::REQUEST;

/// 住所から郵便番号を取得
/// 取得元は郵便局のHR
pub async fn get_post_code(jis_code: String, district: String) -> String {
    let url = "https://www.post.japanpost.jp/cgi-zip/zipcode.php";
    let params = [("pref", &jis_code), ("addr", &district)];
    let html = REQUEST
        .post_response(url, &params)
        .await
        .text()
        .await
        .unwrap();
    html
}
