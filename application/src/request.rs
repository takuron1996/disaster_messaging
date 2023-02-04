use reqwest::{Client, Response};

/// reqwestのClientを使い回すための構造体
pub struct Request {
    pub client: Client,
}

impl Request {
    /// Get通信を実施してresponseを返す
    pub async fn get_response(&self, url: &str) -> Response {
        self.client.get(url).send().await.unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::Request;
    use http::StatusCode;

    fn init_request() -> Request {
        Request {
            client: reqwest::Client::new(),
        }
    }

    /// get通信のステータスコードが200（正常通信）であることをテスト
    #[tokio::test]
    async fn test_get_response_200() {
        let request = init_request();
        let url = "https://crisis.yahoo.co.jp/evacuation/";
        assert_eq!(StatusCode::OK, request.get_response(url).await.status());
    }

    /// get通信のステータスコードが200以外（異常通信）である事のテスト
    #[should_panic]
    #[tokio::test]
    async fn test_get_response_404_panic() {
        let request = init_request();
        let url = "hoge";
        request.get_response(url).await;
    }
}
