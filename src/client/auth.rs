use matrix_sdk::{Client as MatrixClient, ClientConfig};
use serde::Deserialize;
use url::Url;

use super::{context::ClientSettings, CLIENT_ID};
use crate::{fs::DATA_DIRECTORY, handle_login_section};

#[derive(Debug, Clone)]
pub struct AuthCreds {
    pub username:   String,
    pub homeserver: String,
    pub password:   String,
}

pub async fn get_home_server(
    settings: &ClientSettings,
    credentials: &AuthCreds,
) -> Result<Url, String> {
    let url = format!(
        "https://{}/.well-known/matrix/client",
        credentials.homeserver
    );

    let result = handle_login_section!(
        settings,
        reqwest::get(url).await,
        "Unable to connect to home server."
    );

    let text = handle_login_section!(
        settings,
        result.text().await,
        "Unable to get home server response."
    );

    let home_server = handle_login_section!(
        settings,
        serde_json::from_str::<HomeServerResponse>(&text),
        "Unable to parse home server response."
    );

    let url = handle_login_section!(
        settings,
        Url::parse(&home_server.homeserver.url),
        "Home server returned malformed URL."
    );

    Ok(url)
}

pub async fn login(
    settings: &ClientSettings,
    credentials: &AuthCreds,
    home_server: Url,
) -> Result<MatrixClient, String> {
    let store_path = DATA_DIRECTORY.as_ref().unwrap();
    let config = ClientConfig::default().store_path(store_path);

    let client = MatrixClient::new_with_config(home_server, config).unwrap();
    let login = client
        .login(
            &credentials.username.to_lowercase(),
            &credentials.password,
            None,
            Some(&CLIENT_ID),
        )
        .await;

    handle_login_section!(
        settings,
        login,
        "Unable to login with provided credentials."
    );

    Ok(client)
}

#[derive(Deserialize, Debug)]
struct UrlWrapper {
    #[serde(rename = "base_url")]
    url: String,
}

#[derive(Deserialize, Debug)]
struct HomeServerResponse {
    #[serde(rename = "m.homeserver")]
    homeserver:      UrlWrapper,
    #[serde(rename = "m.identity_server")]
    identity_server: UrlWrapper,
}
