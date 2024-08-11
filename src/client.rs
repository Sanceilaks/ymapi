use url::Url;

use crate::{errors::Error, models};

#[derive(Default)]
pub struct ClientBuilder {
    token: Option<String>,
    custom_client: Option<reqwest::Client>,
    base_url: Option<Url>,
}

fn create_http_client(token: &Option<String>) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    match token {
        Some(tok) => {
            headers.append(
                "Authorization",
                std::format!("OAuth {}", tok).parse().unwrap(),
            );
        }
        None => (),
    }
    headers.append(
        "X-Yandex-Music-Client",
        "YandexMusicAndroid/24023621".parse().unwrap(),
    );

    reqwest::ClientBuilder::new()
        .user_agent("Yandex-Music-API")
        .default_headers(headers)
        .build()
        .expect("Cannot create HttpClient")
}

impl ClientBuilder {
    pub fn token(&mut self, token: &str) -> &mut Self {
        self.token = Some(token.to_owned());
        self
    }

    pub fn custom_client(&mut self, client: reqwest::Client) -> &mut Self {
        self.custom_client = Some(client);
        self
    }

    pub fn build(&mut self) -> Client {
        Client {
            http_client: match self.custom_client.to_owned() {
                Some(client) => client.to_owned(),
                None => create_http_client(&self.token).to_owned(),
            },
            base_url: self
                .base_url
                .to_owned()
                .unwrap_or_else(|| Url::parse("https://api.music.yandex.net").unwrap()),
            token: self.token.to_owned(),
        }
    }
}

/// Yandex Music API Client
///
/// # Examples
///
/// ```rust
/// use ymapi::client::Client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::builder().token("token").build();
///     let status = client.account_status().await;
///
///     println!("{:#?}", status);
/// }
/// ```
pub struct Client {
    token: Option<String>,
    http_client: reqwest::Client,
    base_url: Url,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub async fn account_status(&self) -> Result<models::YmApiResponse<serde_json::Value>, Error> {
        self.http_client
            .get(self.base_url.join("/account/status").unwrap())
            .send()
            .await?
            .json()
            .await
            .map(|v| Ok(v))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_http_client_test() {
        let client = create_http_client(&None);
        assert!(client
            .get(Url::parse("https://api.music.yandex.net").unwrap())
            .send()
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_client() {
        let client = Client::builder().build();
        let status = client.account_status().await;

        assert!(status.is_ok(), "Error: {:?}", status);
    }
}
