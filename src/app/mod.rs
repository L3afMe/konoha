use std::io::stdout;

use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent,
        KeyModifiers, MouseEvent,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use tokio::task::JoinHandle;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Terminal,
};

use self::{
    context::{Context, Notification},
    event::Event,
    helper::{draw_help_menu, expand_area, split_text, CrosstermFrame},
    ui::prelude::{
        message::PopupMessageBuilder, new_confirm_popup, AuthenticateMenu,
        Menu, Popup,
    },
};
use crate::{
    app::{
        context::handle_notification,
        event::{handle_event, spawn_event_listener},
        helper::Spacing,
    },
    error::Result,
};

pub mod context;
pub mod event;
mod helper;
pub mod ui;

pub struct App {
    pub client_handle: Option<JoinHandle<()>>,
    pub context:       Context,
    pub menu:          Box<dyn Menu + Send>,
    pub popup:         Option<Popup>,
}

impl App {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            client_handle: None,
            menu: Box::new(AuthenticateMenu::default()),
            popup: None,
        }
    }

    pub fn draw(&mut self, frame: &mut CrosstermFrame) {
        let area = if !self.context.settings.hide_help {
            let help_message = if let Some(popup) = &mut self.popup {
                popup.get_help_message(&self.context)
            } else {
                self.menu.get_help_message(&self.context)
            };

            draw_help_menu(frame, help_message, frame.size())
        } else {
            frame.size()
        };

        let (min_width, min_height) = self.menu.get_minimum_size();
        if min_width > area.width || min_height > area.height {
            let text = split_text(
                "Please resize your screen so there is more space to draw!",
                " ",
                area.width as usize - 2,
            )
            .join("\n");

            let block = Block::default().title("Error").borders(Borders::ALL);
            let error = Paragraph::new(text).block(block);
            frame.render_widget(error, area);
        } else {
            self.menu.draw(frame, area, &self.context);
        }

        if let Some(popup) = &mut self.popup {
            let popup_area = popup.get_area(area);
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let popup_border =
                expand_area(popup_area, Spacing::new(1, 1, 1, 1));
            frame.render_widget(Clear, popup_border);
            frame.render_widget(popup_block, popup_border);

            popup.draw(frame, popup_area, &self.context);
        }
    }

    pub fn on_key_press(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('?') && key.modifiers == KeyModifiers::ALT
        {
            self.context.settings.toggle_help();
            return;
        }

        if key.code == KeyCode::Char('d')
            && key.modifiers == KeyModifiers::CONTROL
        {
            // TODO: Logging
            let _ = self
                .context
                .send_notification(Notification::QuitApplication(true));
            return;
        }

        if let Some(popup) = &mut self.popup {
            popup.on_event(Event::Key(key), &self.context);
        } else {
            self.menu.on_event(Event::Key(key), &self.context);
        }
    }

    pub fn on_mouse(&mut self, event: MouseEvent) {
        if let Some(popup) = &mut self.popup {
            popup.on_event(Event::Mouse(event), &self.context);
        } else {
            self.menu.on_event(Event::Mouse(event), &self.context);
        }
    }

    pub fn on_tick(&mut self) {
        if let Some(popup) = &mut self.popup {
            popup.on_event(Event::Tick, &self.context);
        }

        self.menu.on_event(Event::Tick, &self.context);
    }

    pub fn on_notification(&mut self, notification: Notification) {
        match notification {
            Notification::QuitApplication(show_confirm) => {
                if show_confirm {
                    self.popup = Some(new_confirm_popup(
                        "Are you sure you want to exit?",
                        |ctx| {
                            // TODO: Logging
                            let _ = ctx.send_notification(
                                Notification::QuitApplication(false),
                            );
                        },
                    ))
                } else {
                    self.context.settings.quit_application = true;
                }
            },
            Notification::SetLogin(login) => {
                self.client_handle = Some(self.context.start_client(login))
            },
            Notification::ShowPopup(popup) => self.popup = Some(popup),
            Notification::HidePopup => self.popup = None,
            Notification::SwitchMenu(menu) => self.menu = menu,
            Notification::ClientError(why) => {
                let popup = PopupMessageBuilder::new(why)
                    .set_title(Some("Error"))
                    .to_popup();

                // TODO: Logging
                let _ = self
                    .context
                    .send_notification(Notification::ShowPopup(popup));
            },
        }
    }
}

pub fn start_app() -> Result<()> {
    let (context, noti_rec) = Context::new();

    enable_raw_mode().expect("Unable to enable raw mode.");

    let mut out = stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)
        .expect("Unable to enter new screen.");

    let backend = CrosstermBackend::new(out);
    let mut term = Terminal::new(backend)?;

    let mut app = App::new(context);

    term.clear().expect("Unable to clean terminal.");

    let key_listen_timout = 100;
    let event_rec = spawn_event_listener(key_listen_timout);

    loop {
        term.draw(|f| app.draw(f)).unwrap();

        handle_event(&event_rec.receiver, &mut app);
        handle_notification(&noti_rec, &mut app);

        if app.context.settings.quit_application {
            break;
        }
    }

    let mut out = stdout();
    disable_raw_mode().expect("Unable to disable raw mode.");
    execute!(out, LeaveAlternateScreen, DisableMouseCapture)
        .expect("Unable to restore screen.");

    Ok(())
}
