use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use reqwest::{Client, Response};
use serde::Serialize;

/// reqwestのClientを使い回すための構造体
#[derive(Debug)]
pub struct Request {
    pub client: Client,
}

impl Request {
    /// Get通信を実施してresponseを返す
    pub async fn get_response(url: &str) -> Response {
        REQUEST.get().unwrap().client.get(url).send().await.unwrap()
    }

    /// Post通信を実施してresponseを返す
    pub async fn post_response<T: Serialize + ?Sized>(url: &str, params: &T) -> Response {
        REQUEST
            .get()
            .unwrap()
            .client
            .post(url)
            .form(params)
            .send()
            .await
            .unwrap()
    }
}

/// リクエストの初期化処理
fn init_request() -> Request {
    Request {
        client: reqwest::Client::new(),
    }
}

/// 標準のクライアントを使い回すためのリクエスト
pub static REQUEST: OnceCell<Request> = OnceCell::new();
// pub static REQUEST: Lazy<Request> = Lazy::new(|| init_request());

#[cfg(test)]
mod tests {

    use super::{Request, REQUEST};
    use http::StatusCode;
    use rstest::{fixture, rstest};

    #[fixture]
    #[once]
    fn request_init() {
        REQUEST
            .set(Request {
                client: reqwest::Client::new(),
            })
            .unwrap()
    }

    /// get通信のステータスコードが200（正常通信）である場合のテスト
    #[rstest]
    #[tokio::test]
    async fn test_get_response_200(_request_init: ()) {
        let url = "http://httpbin.org/get";
        assert_eq!(StatusCode::OK, Request::get_response(url).await.status());
    }

    /// get通信のステータスコードが200以外（異常通信）である場合のテスト
    #[rstest]
    #[should_panic]
    #[tokio::test]
    async fn test_get_response_404_panic(_request_init: ()) {
        Request::get_response("").await;
    }

    /// post通信のステータスコードが200（正常通信）である場合のテスト
    #[rstest]
    #[tokio::test]
    async fn test_post_response_200(_request_init: ()) {
        let url = "http://httpbin.org/post";
        assert_eq!(
            StatusCode::OK,
            Request::post_response(url, &[("", "")]).await.status()
        );
    }

    /// post通信のステータスコードが200以外（異常通信）である場合のテスト
    #[rstest]
    #[should_panic]
    #[tokio::test]
    async fn test_post_response_404_panic(_request_init: ()) {
        Request::post_response("", &[("", "")]).await;
    }
}
