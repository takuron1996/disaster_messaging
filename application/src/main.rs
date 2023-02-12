use disaster_messaging::common::get_post_code;

mod disaster_messaging;

#[tokio::main]
async fn main() {
    let html = get_post_code(String::from("01"), String::from("札幌市北区北三十六条西")).await;
    println!("{html}");
}
