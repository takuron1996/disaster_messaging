use reqwest::Client;

mod request;

#[tokio::main]
async fn main() {
    let request = request::Request {
        client: Client::new(),
    };
    let url = "https://crisis.yahoo.co.jp/evacuation/";
    let html = request.get_response(url).await.text().await.unwrap();
    println!("{html}");
}
