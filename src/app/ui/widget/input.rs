use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    layout::{Alignment, Direction, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};

use super::Widget;
use crate::app::{
    context::Context,
    helper::{split_rect, CrosstermFrame},
};

const CURSOR_BLINK_TICKS: u8 = 6;

#[derive(Debug, Clone)]
pub enum ValidationType {
    Manual(bool),
    Functional(fn(String) -> bool),
}

impl Default for ValidationType {
    fn default() -> Self {
        Self::Manual(true)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InputWidget {
    pub value:      String,
    pub max_len:    usize,
    pub secret:     bool,
    pub validation: ValidationType,
    pub selected:   bool,
    pub cursor_pos: usize,
    pub scroll_pos: usize,
    tick_count:     u8,
}

#[allow(dead_code)]
impl InputWidget {
    pub fn set_value<T: ToString>(&mut self, value: T) -> &mut Self {
        self.value = value.to_string();
        self
    }

    pub fn set_max_len(&mut self, len: usize) -> &mut Self {
        self.max_len = len;
        self
    }

    pub fn set_secret(&mut self, secret: bool) -> &mut Self {
        self.secret = secret;
        self
    }

    pub fn set_validation(&mut self, validation: ValidationType) -> &mut Self {
        self.validation = validation;
        self
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        if selected != self.selected {
            self.on_focus(selected);
        }
        self
    }

    pub fn set_cursor_pos(&mut self, pos: usize) -> &mut Self {
        self.cursor_pos = pos;
        self
    }

    pub fn set_scroll_pos(&mut self, pos: usize) -> &mut Self {
        self.scroll_pos = pos;
        self
    }

    pub fn is_valid(&mut self) -> bool {
        match self.validation {
            ValidationType::Manual(value) => value,
            ValidationType::Functional(func) => func(self.value.clone()),
        }
    }
}

impl Widget for InputWidget {
    fn render(&mut self, area: Rect, frame: &mut CrosstermFrame) {
        let max_len = area.width as usize;

        let mut value_text = if self.secret {
            "*".repeat(self.value.len())
        } else {
            self.value.clone()
        };

        if self.scroll_pos > 0 {
            value_text = value_text
                [self.scroll_pos..max_len.min(value_text.len())]
                .to_string();
        }

        let placeholder_text = "_".repeat(
            // use min to ensure value isn't over max_len
            max_len - value_text.len().min(max_len),
        );

        let mut text = format!("{}{}", value_text, placeholder_text);

        if self.selected && self.tick_count < CURSOR_BLINK_TICKS {
            // TODO: Better way to do this
            let mut chars: Vec<char> = text.chars().collect();
            let cursor_pos = self.cursor_pos as i32 - self.scroll_pos as i32;

            // Ensure that no panic
            if cursor_pos >= 0
                && chars.get(cursor_pos as usize).cloned().is_some()
            {
                chars.remove(cursor_pos as usize);
                chars.insert(cursor_pos as usize, '█');

                text = String::default();
                for c in chars {
                    text += &c.to_string();
                }
            }
        }

        let block =
            Paragraph::new(text).style(Style::default().fg(
                if self.is_valid() { Color::Indexed(8) } else { Color::Red },
            ));

        frame.render_widget(block, area);
    }

    fn on_key(&mut self, _ctx: &Context, key: KeyEvent) {
        if self.selected {
            match key.code {
                KeyCode::Char(key) => {
                    let split = (
                        self.value
                            .chars()
                            .take(self.cursor_pos)
                            .collect::<String>(),
                        self.value
                            .chars()
                            .skip(self.cursor_pos)
                            .collect::<String>(),
                    );
                    self.value = format!("{}{}{}", split.0, key, split.1);

                    self.cursor_pos += 1;
                },
                KeyCode::Left => self.cursor_pos = self.cursor_pos.max(1) - 1,
                KeyCode::Right => {
                    self.cursor_pos =
                        (self.cursor_pos + 1).min(self.value.len())
                },
                KeyCode::Backspace => {
                    if self.cursor_pos != 0 {
                        self.cursor_pos -= 1;
                        self.value.remove(self.cursor_pos);
                    }
                },
                KeyCode::Delete => {
                    if self.value.len() > self.cursor_pos {
                        self.value.remove(self.cursor_pos);
                    }
                },
                _ => {
                    // Return so tick count doesn't get
                    // reset
                    return;
                },
            }

            self.tick_count = 0;
        }
    }

    fn on_tick(&mut self, _ctx: &Context) {
        if self.selected {
            self.tick_count += 1;
            self.tick_count %= CURSOR_BLINK_TICKS * 2;
        } else {
            self.tick_count = 0;
        }
    }

    fn on_focus(&mut self, arrive: bool) {
        self.selected = arrive;
    }

    fn has_focus(&mut self) -> bool {
        self.selected
    }
}

#[derive(Debug, Clone)]
pub struct LabeledInputWidget {
    pub label:            String,
    pub label_align:      Alignment,
    pub split_percentage: u16,
    pub input:            InputWidget,
}

#[allow(dead_code)]
impl LabeledInputWidget {
    pub fn new<T: ToString>(label: T) -> Self {
        Self {
            label:            label.to_string(),
            label_align:      Alignment::Left,
            split_percentage: 40,
            input:            InputWidget::default(),
        }
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        if selected != self.input.selected {
            self.on_focus(selected);
        }
        self
    }

    pub fn set_alignment(&mut self, alignment: Alignment) -> &mut Self {
        self.label_align = alignment;
        self
    }

    pub fn set_split(&mut self, percentage: u16) -> &mut Self {
        self.split_percentage = percentage;
        self
    }

    pub fn set_input(
        &mut self,
        input_fn: fn(&mut InputWidget) -> &mut InputWidget,
    ) -> &mut Self {
        self.input = input_fn(&mut self.input).clone();
        self
    }

    pub fn set_validation(&mut self, validation: ValidationType) -> &mut Self {
        self.input.set_validation(validation);
        self
    }

    pub fn set_secret(&mut self, secret: bool) -> &mut Self {
        self.input.set_secret(secret);
        self
    }
}

impl Widget for LabeledInputWidget {
    fn on_tick(&mut self, ctx: &Context) {
        self.input.on_tick(ctx);
    }

    fn on_key(&mut self, ctx: &Context, key: KeyEvent) {
        self.input.on_key(ctx, key);
    }

    fn render(&mut self, area: Rect, frame: &mut CrosstermFrame) {
        let split =
            split_rect(self.split_percentage, Direction::Horizontal, area);

        let label =
            Paragraph::new(self.label.clone()).alignment(self.label_align);
        frame.render_widget(label, split[0]);
        self.input.render(split[1], frame);
    }

    fn on_focus(&mut self, arrive: bool) {
        self.input.on_focus(arrive);
    }

    fn has_focus(&mut self) -> bool {
        self.input.selected
    }
}
