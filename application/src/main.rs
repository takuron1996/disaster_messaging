use reqwest::Client;

mod logging;
mod request;

#[tokio::main]
async fn main() {
    logging::init_logger();

    let request = request::Request {
        client: Client::new(),
    };
    let url = "https://crisis.yahoo.co.jp/evacuation/";
    let html = request.get_response(url).await.text().await.unwrap();
    log::info!("{html}");
}
