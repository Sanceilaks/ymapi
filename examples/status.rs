extern crate tokio;
extern crate ymapi;

#[tokio::main]
async fn main() {
    let client = ymapi::client::Client::builder()
        .token(env!("YANDEX_MUSIC_TOKEN"))
        .build()
        .await;
    let status = client.account_status().await.unwrap();

    println!("{:#?}", status);
}
