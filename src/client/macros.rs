#[macro_export]
macro_rules! handle_login {
    ($client:expr, $val:expr, $msg:expr) => {{
        // TODO: Logging
        let menu = LoadingMenu::new($msg);
        let notification = Notification::SwitchMenu(Box::new(menu));
        let _ = $client.context.send_notification(notification);

        match $val.await {
            Ok(val) => val,
            Err(why) => {
                // TODO: Logging
                let menu = AuthenticateMenu::new($client.credentials.clone());
                let notification = Notification::SwitchMenu(Box::new(menu));
                let _ = $client.context.send_notification(notification);

                // TODO: Logging
                let notification = Notification::ClientError(why);
                let _ = $client.context.send_notification(notification);

                return;
            },
        }
    }};
}

#[macro_export]
macro_rules! handle_login_section {
    ($ctx:expr, $val:expr, $msg:expr) => {
        match $val {
            Ok(value) => value,
            Err(why) => {
                return Err(if $ctx.verbose {
                    format!("{}\n{}", $msg, why)
                } else {
                    $msg.to_string()
                });
            },
        }
    };
}
