use std::{
    sync::mpsc::{self, Receiver, SendError, Sender},
    time::Duration,
};

use tokio::task::JoinHandle;

use super::{
    ui::prelude::{Menu, Popup},
    App,
};
use crate::client::{auth::AuthCreds, Client, ClientNotification};

pub enum Notification {
    QuitApplication(bool),
    SetLogin(AuthCreds),
    ShowPopup(Popup),
    HidePopup,
    SwitchMenu(Box<dyn Menu + Send>),
    ClientError(String),
}

#[derive(Debug, Clone, Default)]
pub struct ContextSettings {
    pub hide_help:        bool,
    pub quit_application: bool,
    pub login_details:    Option<AuthCreds>,
}

impl ContextSettings {
    pub fn toggle_help(&mut self) {
        self.hide_help = !self.hide_help;
    }
}

pub struct Context {
    notification_sender: Sender<Notification>,
    client_notification_sender: Option<Sender<ClientNotification>>,
    pub settings: ContextSettings,
}

impl Context {
    pub fn new() -> (Self, Receiver<Notification>) {
        let (notification_sender, notification_rec) = mpsc::channel();

        let this = Self {
            notification_sender,
            client_notification_sender: None,
            settings: ContextSettings::default(),
        };

        (this, notification_rec)
    }

    pub fn send_client_notification(
        &self,
        notification: ClientNotification,
    ) -> Result<(), SendError<ClientNotification>> {
        if let Some(sender) = &self.client_notification_sender {
            sender.send(notification)
        } else {
            Ok(())
        }
    }

    pub fn send_notification(
        &self,
        notification: Notification,
    ) -> Result<(), SendError<Notification>> {
        self.notification_sender.send(notification)
    }

    pub fn start_client(&mut self, credentials: AuthCreds) -> JoinHandle<()> {
        let sender = self.notification_sender.clone();
        let (mut client, sender) = Client::new(credentials, sender);
        let handle = tokio::task::spawn(async move { client.login().await });

        self.client_notification_sender = Some(sender);
        handle
    }
}

pub fn handle_notification(receiver: &Receiver<Notification>, app: &mut App) {
    if let Ok(notification) = receiver.recv_timeout(Duration::from_millis(0)) {
        app.on_notification(notification);
    }
}
