use std::sync::mpsc::{self, Sender};

use clap::{crate_name, crate_version};
use lazy_static::lazy_static;
use matrix_sdk::SyncSettings;

use self::{
    auth::{get_home_server, login, AuthCreds},
    context::Context,
};
use crate::{
    app::{
        context::Notification,
        ui::prelude::{AuthenticateMenu, LoadingMenu},
    },
    handle_login,
};

pub mod auth;
mod context;
mod event;
pub mod macros;

pub enum ClientNotification {
    Test,
}

lazy_static! {
    pub static ref CLIENT_ID: String = format!(
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

pub struct Client {
    credentials: AuthCreds,
    pub context: Context,
}

impl Client {
    pub fn new(
        credentials: AuthCreds,
        sender: Sender<Notification>,
    ) -> (Self, Sender<ClientNotification>) {
        let (app_sender, receiver) = mpsc::channel();
        let context = Context::new(sender, receiver);

        (
            Self {
                credentials,
                context,
            },
            app_sender,
        )
    }

    pub async fn login(&mut self) {
        let settings = &self.context.settings;

        let home_server = handle_login!(
            self,
            get_home_server(settings, &self.credentials),
            "Fetching home server"
        );

        let client = handle_login!(
            self,
            login(&settings, &self.credentials, home_server),
            "Logging in"
        );

        // TODO: Logging
        let menu = LoadingMenu::new("Syncing data (this may take a while)");
        let notification = Notification::SwitchMenu(Box::new(menu));
        let _ = self.context.send_notification(notification);

        client.sync(SyncSettings::default()).await;
    }
}
