use super::request::Request;

async fn get_post_code(jis_code: String, district: String) -> String {
    let url = "https://www.post.japanpost.jp/cgi-zip/zipcode.php";
    let params = [("pref", &jis_code), ("addr", &district)];
    let html = Request::post_response(url, &params)
        .await
        .text()
        .await
        .unwrap();
    html
}
