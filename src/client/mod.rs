use std::sync::mpsc::{self, Receiver, SendError, Sender};

use clap::{crate_name, crate_version};
use lazy_static::lazy_static;
use matrix_sdk::{Client as MatrixClient, SyncSettings};
use serde::Deserialize;
use url::Url;

use self::auth::AuthCreds;
use crate::{
    app::{
        context::Notification,
        ui::prelude::{AuthenticateMenu, LoadingMenu},
    },
    handle_login,
};

pub mod auth;
mod event;
pub mod macros;

pub enum ClientNotification {
    Test,
}

lazy_static! {
    static ref CLIENT_ID: String = format!(
        "{} v{} ({})",
        crate_name!(),
        crate_version!(),
        if cfg!(windows) {
            "Windows"
        } else if cfg!(macos) {
            "macOS"
        } else {
            "Linux"
        }
    );
}

#[derive(Debug, Default, Clone)]
pub struct ClientSettings {
    pub verbose: bool,
}

pub struct Client {
    credentials:  AuthCreds,
    sender:       Sender<Notification>,
    receiver:     Receiver<ClientNotification>,
    pub settings: ClientSettings,
}

impl Client {
    pub fn new(
        credentials: AuthCreds,
        sender: Sender<Notification>,
    ) -> (Self, Sender<ClientNotification>) {
        let (app_sender, receiver) = mpsc::channel();
        (
            Self {
                credentials,
                sender,
                receiver,
                settings: ClientSettings::default(),
            },
            app_sender,
        )
    }

    pub async fn login(&mut self) {
        // TODO: Logging
        let _ = self.send_notification(Notification::SwitchMenu(Box::new(
            LoadingMenu::new("Fetching home server"),
        )));

        let url = format!(
            "https://{}/.well-known/matrix/client",
            self.credentials.homeserver
        );

        let result = handle_login!(
            self,
            reqwest::get(url).await,
            "Unable to connect to home server."
        );

        let text = handle_login!(
            self,
            result.text().await,
            "Unable to get home server response."
        );

        let respone = handle_login!(
            self,
            serde_json::from_str::<HomeServerResponse>(&text),
            "Unable to parse home server response."
        );

        let new_screen = LoadingMenu::new("Logging in");
        // TODO: Logging
        let _ = self
            .send_notification(Notification::SwitchMenu(Box::new(new_screen)));

        let url = handle_login!(
            self,
            Url::parse(&respone.homeserver.url),
            "Home server returned malformed URL."
        );

        let client = MatrixClient::new(url).unwrap();
        let login = client.login(
            &self.credentials.username,
            &self.credentials.password,
            None,
            Some(&CLIENT_ID),
        ).await;

        handle_login!(
            self,
            login,
            "Unable to login with provided credentials."
        );

        self.credentials.homeserver = respone.homeserver.url;

        let new_screen = LoadingMenu::new("Syncing data");
        // TODO: Logging
        let _ = self
            .send_notification(Notification::SwitchMenu(Box::new(new_screen)));
        
        let sync = client.sync(SyncSettings::new()).await;
    }

    pub fn send_notification(
        &self,
        notification: Notification,
    ) -> Result<(), SendError<Notification>> {
        self.sender.send(notification)
    }
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
