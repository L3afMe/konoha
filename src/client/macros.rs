#[macro_export]
macro_rules! handle_login {
    ($self:ident, $x:expr, $y:expr) => {
        match $x {
            Ok(value) => value,
            Err(why) => {
                // TODO: Logging
                let _ = $self.send_notification(Notification::SwitchMenu(
                    Box::new(AuthenticateMenu::new($self.credentials.clone())),
                ));
                // TODO: Logging
                let _ = $self.send_notification(Notification::ClientError(
                    if $self.settings.verbose {
                        format!("{}\n{}", $y, why)
                    } else {
                        $y.to_string()
                    },
                ));
                return;
            },
        }
    };
}
