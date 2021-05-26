use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lazy_static::lazy_static;
use regex::Regex;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};

use super::Menu;
use crate::{
    app::{
        context::{Context, Notification},
        event::Event,
        helper::{self, split_rect, CenterPosition, CrosstermFrame},
        ui::prelude::{
            message::PopupMessageBuilder, ButtonWidget, LabeledInputWidget,
            ValidationType, Widget,
        },
    },
    client::auth::AuthCreds,
};

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(
        "^@?(?P<username>[a-zA-Z0-9_\\-\\.=/]{2,16}):(?P<homeserver>([a-zA-Z\\d-]+\\.\
         ){1,}[a-z]+)$"
    )
    .unwrap();
}

#[derive(Clone)]
pub struct AuthenticateMenu {
    focus_index: u8,
    username:    LabeledInputWidget,
    password:    LabeledInputWidget,
    submit:      ButtonWidget,
}

impl Default for AuthenticateMenu {
    fn default() -> Self {
        let username = LabeledInputWidget::new("Username")
            .set_selected(true)
            .set_validation(ValidationType::Functional(|username| {
                USERNAME_REGEX.is_match(&username)
            }))
            .to_owned();

        let password = LabeledInputWidget::new("Password")
            .set_secret(true)
            .set_validation(ValidationType::Functional(|password| {
                !password.is_empty()
            }))
            .to_owned();

        let submit = ButtonWidget::new("Login", |_| {});

        Self {
            focus_index: 0,
            username,
            password,
            submit,
        }
    }
}

impl Menu for AuthenticateMenu {
    fn on_event(&mut self, event: Event, ctx: &Context) {
        match event {
            Event::Tick => self.on_tick(ctx),
            Event::Key(key) => self.handle_key(key, ctx),
            _ => {},
        }
    }

    fn get_help_message(
        &mut self,
        _ctx: &Context,
    ) -> Vec<(KeyModifiers, KeyCode, String)> {
        vec![
            (KeyModifiers::NONE, KeyCode::Up, "Select up".to_string()),
            (KeyModifiers::NONE, KeyCode::Down, "Select down".to_string()),
            (
                KeyModifiers::NONE,
                KeyCode::Enter,
                "Submit login".to_string(),
            ),
        ]
    }

    fn draw(
        &mut self,
        frame: &mut CrosstermFrame,
        max_size: Rect,
        ctx: &Context,
    ) {
        if max_size.width >= 42 {
            // If help menu is shown, lower the max
            // size by 3 so that it
            // doesn't move when toggling the menu
            let max_size = if !ctx.settings.hide_help {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(1)])
                    .split(max_size)[1]
            } else {
                max_size
            };

            let size = helper::centered_rect(
                CenterPosition::AbsoluteInner(40, 5),
                max_size,
            );

            let button_chunk = split_rect(
                40,
                Direction::Horizontal,
                helper::centered_line(36, 1, 3, size),
            )[1];

            self.username
                .render(helper::centered_line(36, 1, 1, size), frame);
            self.password
                .render(helper::centered_line(36, 1, 2, size), frame);
            self.submit.render(button_chunk, frame);

            let frame_block = Block::default()
                .title("Login to Matrix")
                .borders(Borders::ALL);
            frame.render_widget(frame_block, size);
        } else {
            let frame_block = Paragraph::new(
                split_text(
                    "Please resize your screen so there is more space to draw!"
                        .to_string(),
                    (max_size.width as usize).max(3) - 2,
                )
                .join("\n"),
            )
            .block(Block::default().title("Error").borders(Borders::ALL));
            frame.render_widget(frame_block, frame.size());
        }
    }
}

impl AuthenticateMenu {
    pub fn new(credentials: AuthCreds) -> Self {
        let mut default = Self::default();
        default.username.input.set_value(format!(
            "@{}:{}",
            credentials.username, credentials.homeserver
        ));
        default.password.input.set_value(credentials.password);

        default
    }

    fn on_tick(&mut self, ctx: &Context) {
        self.username.on_tick(ctx);
        self.password.on_tick(ctx);
        self.submit.on_tick(ctx);
    }

    fn handle_key(&mut self, key: KeyEvent, ctx: &Context) {
        match key.code {
            KeyCode::Up => {
                if self.focus_index == 0 {
                    self.focus_index = 2;
                } else {
                    self.focus_index -= 1;
                }

                self.username.set_selected(self.focus_index == 0);
                self.password.set_selected(self.focus_index == 1);
                self.submit.set_selected(self.focus_index == 2);

                return;
            },
            KeyCode::Down => {
                self.focus_index += 1;
                self.focus_index %= 3;

                self.username.set_selected(self.focus_index == 0);
                self.password.set_selected(self.focus_index == 1);
                self.submit.set_selected(self.focus_index == 2);

                return;
            },
            KeyCode::Enter => {
                let error = if !self.username.input.is_valid() {
                    Some("Username should match '@user:domain'.")
                } else if !self.password.input.is_valid() {
                    Some("No password specified.")
                } else {
                    None
                };

                if let Some(msg) = error {
                    let mut popup_builder = PopupMessageBuilder::new(msg);
                    let popup = popup_builder
                        .set_title(Some("Invalid Credentials"))
                        .set_message_align(Alignment::Center)
                        .to_popup();
                    // TODO: Logging
                    let _ =
                        ctx.send_notification(Notification::ShowPopup(popup));

                    return;
                }

                let capture = USERNAME_REGEX
                    .captures(&self.username.input.value)
                    .expect("Couldn't capture username regex."); // This should never happen as self.username would be invalid
                let un_group = capture.name("username").unwrap();
                let username = un_group.as_str().to_string();

                let hs_group = capture.name("homeserver").unwrap();
                let homeserver = hs_group.as_str().to_string();

                let credentials = AuthCreds {
                    username,
                    homeserver,
                    password: self.password.input.value.clone(),
                };

                // TODO: Logging
                let _ =
                    ctx.send_notification(Notification::SetLogin(credentials));
            },
            KeyCode::Tab => {
                let mut popup_builder =
                    PopupMessageBuilder::new("Test popup lol");
                let popup = popup_builder
                    .set_title(Some("Test"))
                    .set_title_align(Alignment::Right)
                    .to_popup();
                // TODO: Logging
                let _ = ctx.send_notification(Notification::ShowPopup(popup));
            },
            _ => {},
        }

        self.username.on_key(ctx, key);
        self.password.on_key(ctx, key);
    }
}

fn split_text(text: String, size: usize) -> Vec<String> {
    let mut output = Vec::new();
    let mut input_remaining = text;

    loop {
        if input_remaining.len() < size {
            output.push(input_remaining);
            break;
        } else {
            let (split, remaining) =
                input_remaining.split_at(input_remaining.len().min(size));
            if split.contains(' ') {
                let (split, split_remaining) = split.rsplit_once(' ').unwrap();
                output.push(split.to_string().trim_matches(' ').to_string());
                input_remaining = (split_remaining.to_string() + remaining)
                    .trim_matches(' ')
                    .to_string();
            } else if remaining.starts_with(' ') || split.ends_with(' ') {
                output.push(split.to_string().trim_matches(' ').to_string());
                input_remaining =
                    remaining.to_string().trim_matches(' ').to_string();
            }
        }
    }

    output
}
