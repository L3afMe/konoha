use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Paragraph, Wrap},
};

use super::{Popup, PopupArea, PopupPosition};
use crate::app::{context::Context, event::Event, helper::{shrink_area, CrosstermFrame, Spacing}, ui::prelude::Menu};

#[derive(Debug, Clone)]
pub struct PopupMessageBuilder {
    title:           Option<String>,
    title_align:     Alignment,
    message:         String,
    message_align:   Alignment,
    position:        PopupPosition,
    message_padding: Spacing,
    title_padding:   Spacing,
}

#[allow(dead_code)]
impl PopupMessageBuilder {
    pub fn new<T>(message: T) -> Self
    where
        T: ToString, {
        Self {
            title:           None,
            title_align:     Alignment::Center,
            message:         message.to_string(),
            message_align:   Alignment::Left,
            position:        PopupPosition::Center,
            message_padding: Spacing::new(1, 1, 4, 4),
            title_padding:   Spacing::default(),
        }
    }

    pub fn to_popup(&self) -> Popup {
        let area = {
            let (message_width, message_height) =
                format_padding(&self.message, self.message_padding);
            let (title_width, title_height) = if let Some(title) = &self.title {
                format_padding(title, self.title_padding)
            } else {
                (0, 0)
            };

            let width = title_width.max(message_width);
            let height = title_height + message_height;

            PopupArea::Absolute(width, height, self.position)
        };

        Popup {
            menu: Box::new(MessageMenu {
                message:         self.message.clone(),
                message_align:   self.message_align,
                message_padding: self.message_padding,
                title:           self.title.clone(),
                title_align:     self.title_align,
                title_padding:   self.title_padding,
            }),
            area,
        }
    }

    pub fn set_title<T>(&mut self, title: Option<T>) -> &mut Self
    where
        T: ToString, {
        self.title = title.map(|title| title.to_string());
        self
    }

    pub fn set_title_align(&mut self, title_align: Alignment) -> &mut Self {
        self.title_align = title_align;
        self
    }

    pub fn set_message<T>(&mut self, message: T) -> &mut Self
    where
        T: ToString, {
        self.message = message.to_string();
        self
    }

    pub fn set_message_align(&mut self, message_align: Alignment) -> &mut Self {
        self.message_align = message_align;
        self
    }

    pub fn set_message_padding(
        &mut self,
        top: u16,
        bottom: u16,
        left: u16,
        right: u16,
    ) -> &mut Self {
        self.message_padding = Spacing {
            top,
            bottom,
            left,
            right,
        };
        self
    }

    pub fn set_title_padding(
        &mut self,
        top: u16,
        bottom: u16,
        left: u16,
        right: u16,
    ) -> &mut Self {
        self.title_padding = Spacing {
            top,
            bottom,
            left,
            right,
        };
        self
    }
}

struct MessageMenu {
    message:         String,
    message_align:   Alignment,
    title:           Option<String>,
    title_align:     Alignment,
    message_padding: Spacing,
    title_padding:   Spacing,
}

impl Menu for MessageMenu {
    fn draw(
        &mut self,
        frame: &mut CrosstermFrame,
        mut max_size: Rect,
        _ctx: &Context,
    ) {
        if let Some(title) = &self.title {
            let (_title_width, title_height) =
                format_padding(title, self.title_padding);
            let split = Layout::default()
                .constraints([
                    Constraint::Length(title_height),
                    Constraint::Min(1),
                ])
                .direction(tui::layout::Direction::Vertical)
                .split(max_size);

            let title_block = Paragraph::new(title.clone())
                .wrap(Wrap {
                    trim: true,
                })
                .alignment(self.title_align)
                .style(Style::default().add_modifier(Modifier::BOLD));
            frame.render_widget(
                title_block,
                shrink_area(split[0], self.title_padding),
            );

            max_size = split[1];
        }

        let block = Paragraph::new(self.message.clone())
            .wrap(Wrap {
                trim: false,
            })
            .alignment(self.message_align);
        frame.render_widget(block, shrink_area(max_size, self.message_padding));
    }

    fn on_event(&mut self, event: Event, ctx: &Context) {
        if let Event::Key(key) = event {
            if key.code == KeyCode::Esc {
                // TODO: Logging
                let _ = ctx.send_notification(
                    crate::app::context::Notification::HidePopup,
                );
            }
        }
    }

    fn get_help_message(
        &mut self,
        _ctx: &Context,
    ) -> Vec<(KeyModifiers, KeyCode, String)> {
        vec![(KeyModifiers::NONE, KeyCode::Esc, "Close popup".to_string())]
    }

    fn get_minimum_size(&mut self) -> (u16, u16) {
        // TODO: Padding
        (0, 0)
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
