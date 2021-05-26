use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use lazy_static::lazy_static;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Paragraph, Wrap},
};

use super::{Popup, PopupArea, PopupPosition};
use crate::app::{
    context::{Context, Notification},
    event::Event,
    helper::{shrink_area, split_rect, CrosstermFrame, Spacing},
    ui::prelude::{ButtonWidget, Menu, Widget},
};

lazy_static! {
    static ref CONFIRM_TITLE: String = "Confirm".to_string();
    static ref TITLE_SPACING: Spacing = Spacing::new(0, 0, 8, 8);
    static ref MESSAGE_SPACING: Spacing = Spacing::new(1, 1, 4, 4);
}

pub fn new_confirm_popup<T: ToString>(
    message: T,
    callback: fn(&Context),
) -> Popup {
    let area = {
        let (message_width, message_height) =
            format_padding(&message.to_string(), TITLE_SPACING.to_owned());
        let (title_width, title_height) =
            format_padding(&CONFIRM_TITLE, MESSAGE_SPACING.to_owned());

        let width = title_width.max(message_width);
        // 1 for button line
        let height = title_height + message_height + 1;

        PopupArea::Absolute(width, height, PopupPosition::Center)
    };

    Popup {
        menu: Box::new(ConfirmMenu::new(message.to_string(), callback)),
        area,
    }
}

struct ConfirmMenu {
    message:        String,
    cancel_button:  ButtonWidget,
    confirm_button: ButtonWidget,
    focus_index:    u8,
}

impl ConfirmMenu {
    fn new(message: String, callback: fn(&Context)) -> Self {
        let cancel_button = ButtonWidget::new("Cancel", |_| {})
            .set_selected(true)
            .to_owned();
        let confirm_button = ButtonWidget::new("Confirm", callback);

        Self {
            message,
            cancel_button,
            confirm_button,
            focus_index: 0,
        }
    }

    fn handle_key(&mut self, key: KeyEvent, ctx: &Context) {
        match key.code {
            KeyCode::Left => {
                if self.focus_index == 0 {
                    self.focus_index = 1;
                } else {
                    self.focus_index -= 1;
                }

                self.cancel_button.set_selected(self.focus_index == 0);
                self.confirm_button.set_selected(self.focus_index == 1);
            },
            KeyCode::Right => {
                self.focus_index += 1;
                self.focus_index %= 2;

                self.cancel_button.set_selected(self.focus_index == 0);
                self.confirm_button.set_selected(self.focus_index == 1);
            },
            KeyCode::Enter => {
                self.cancel_button.on_key(ctx, key);
                self.confirm_button.on_key(ctx, key);

                // TODO: Logging
                let _ = ctx.send_notification(Notification::HidePopup);
            },
            _ => {},
        }
    }
}

impl Menu for ConfirmMenu {
    fn on_event(&mut self, event: Event, ctx: &Context) {
        if let Event::Key(key) = event {
            self.handle_key(key, ctx);
        }
    }

    fn get_help_message(
        &mut self,
        _ctx: &Context,
    ) -> Vec<(KeyModifiers, KeyCode, String)> {
        vec![
            (KeyModifiers::NONE, KeyCode::Left, "Select left".to_string()),
            (
                KeyModifiers::NONE,
                KeyCode::Right,
                "Select right".to_string(),
            ),
            (
                KeyModifiers::NONE,
                KeyCode::Enter,
                "Confirm selection".to_string(),
            ),
        ]
    }

    fn draw(
        &mut self,
        frame: &mut CrosstermFrame,
        mut max_size: Rect,
        _ctx: &Context,
    ) {
        let (_title_width, title_height) =
            format_padding(&CONFIRM_TITLE.to_owned(), TITLE_SPACING.to_owned());
        let split = Layout::default()
            .constraints([
                Constraint::Length(title_height),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .direction(tui::layout::Direction::Vertical)
            .split(max_size);

        let title_block = Paragraph::new(CONFIRM_TITLE.to_owned())
            .wrap(Wrap {
                trim: true,
            })
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(
            title_block,
            shrink_area(split[0], TITLE_SPACING.to_owned()),
        );

        max_size = split[1];

        let block = Paragraph::new(self.message.clone())
            .wrap(Wrap {
                trim: false,
            })
            .alignment(Alignment::Center);
        frame.render_widget(
            block,
            shrink_area(max_size, MESSAGE_SPACING.to_owned()),
        );

        let button_split = split_rect(50, Direction::Horizontal, split[2]);
        self.confirm_button.render(button_split[0], frame);
        self.cancel_button.render(button_split[1], frame);
    }
}

fn format_padding(message: &str, padding: Spacing) -> (u16, u16) {
    let lines = message.split('\n');
    let longest_line = lines
        .clone()
        .map(|line| line.len())
        .reduce(|curr_len, new_len| new_len.max(curr_len))
        .unwrap_or(0);

    let width = longest_line as u16 + padding.left + padding.right;
    let height = lines.count() as u16 + padding.top + padding.bottom;

    (width, height)
}
