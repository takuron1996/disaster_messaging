use once_cell::sync::Lazy;
use reqwest::{Client, Response};
use serde::Serialize;

/// reqwestのClientを使い回すための構造体
#[derive(Debug)]
pub struct Request {
    pub client: Client,
}

impl Request {
    /// Get通信を実施してresponseを返す
    pub async fn get_response(&self, url: &str) -> Result<Response, reqwest::Error> {
        self.client.get(url).send().await
    }

    /// Post通信を実施してresponseを返す
    pub async fn post_response<T: Serialize + ?Sized>(
        &self,
        url: &str,
        params: &T,
    ) -> Result<Response, reqwest::Error> {
        self.client.post(url).form(params).send().await
    }
}

/// リクエストの初期化処理
fn init_request() -> Request {
    Request {
        client: reqwest::Client::new(),
    }
}

/// 標準のクライアントを使い回すためのリクエスト
pub static REQUEST: Lazy<Request> = Lazy::new(init_request);

#[cfg(test)]
mod tests {

    use super::REQUEST;
    use http::StatusCode;

    /// get通信のステータスコードが200（正常通信）である場合のテスト
    #[tokio::test]
    async fn test_get_response_200() {
        let url = "http://httpbin.org/get";
        assert_eq!(
            StatusCode::OK,
            REQUEST.get_response(url).await.unwrap().status()
        );
    }

    /// get通信のステータスコードが200以外（異常通信）である場合のテスト
    #[tokio::test]
    async fn test_get_response_404_panic() {
        assert!(REQUEST.get_response("").await.is_err());
    }

    /// post通信のステータスコードが200（正常通信）である場合のテスト
    #[tokio::test]
    async fn test_post_response_200() {
        let url = "http://httpbin.org/post";
        assert_eq!(
            StatusCode::OK,
            REQUEST
                .post_response(url, &[("", "")])
                .await
                .unwrap()
                .status()
        );
    }

    /// post通信のステータスコードが200以外（異常通信）である場合のテスト
    #[tokio::test]
    async fn test_post_response_404_panic() {
        assert!(REQUEST.post_response("", &[("", "")]).await.is_err());
    }
}
