use std::sync::mpsc::{Receiver, SendError, Sender};

use super::ClientNotification;
use crate::app::context::Notification;

#[derive(Debug, Default, Clone)]
pub struct ClientSettings {
    pub verbose: bool,
}

pub struct Context {
    sender:       Sender<Notification>,
    receiver:     Receiver<ClientNotification>,
    pub settings: ClientSettings,
}

impl Context {
    pub fn new(
        sender: Sender<Notification>,
        receiver: Receiver<ClientNotification>,
    ) -> Self {
        Self {
            sender,
            receiver,
            settings: ClientSettings::default(),
        }
    }

    pub fn send_notification(
        &self,
        notification: Notification,
    ) -> Result<(), SendError<Notification>> {
        self.sender.send(notification)
    }
}
