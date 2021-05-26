use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

use super::Widget;
use crate::app::{context::Context, helper::CrosstermFrame};

#[derive(Clone)]
pub struct ButtonWidget {
    pub text:           String,
    pub submit_fn:      fn(&Context),
    pub selected:       bool,
    pub enabled:        bool,
    pub inner_padding:  usize,
    pub outter_padding: usize,
    pub alignment:      Alignment,
}

#[allow(dead_code)]
impl ButtonWidget {
    pub fn new<T: ToString>(text: T, submit_fn: fn(&Context)) -> Self {
        Self {
            text: text.to_string(),
            submit_fn,
            ..Default::default()
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        self
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        if selected != self.selected {
            self.on_focus(selected);
        }
        self
    }

    pub fn set_inner_padding(&mut self, inner_padding: usize) -> &mut Self {
        self.inner_padding = inner_padding;
        self
    }

    pub fn set_outter_padding(&mut self, outter_padding: usize) -> &mut Self {
        self.outter_padding = outter_padding;
        self
    }

    pub fn set_alignment(&mut self, alignment: Alignment) -> &mut Self {
        self.alignment = alignment;
        self
    }
}

impl Default for ButtonWidget {
    fn default() -> Self {
        Self {
            text:           String::default(),
            submit_fn:      |_| {},
            selected:       bool::default(),
            enabled:        true,
            inner_padding:  usize::default(),
            outter_padding: usize::default(),
            alignment:      Alignment::Center,
        }
    }
}

impl Widget for ButtonWidget {
    fn render(&mut self, area: Rect, frame: &mut CrosstermFrame) {
        let inner_padding = " ".repeat(self.inner_padding);
        let outter_padding = " ".repeat(self.outter_padding);

        let mut label = format!(
            "{}[{}{}{}]{}",
            outter_padding,
            inner_padding,
            self.text,
            inner_padding,
            outter_padding
        );

        if label.len() < area.width as usize {
            let difference = area.width as usize - label.len();
            match self.alignment {
                Alignment::Left => {
                    label = label + &" ".repeat(difference);
                },
                Alignment::Center => {
                    let left = if difference % 2 == 0 {
                        difference / 2
                    } else {
                        (difference + 1) / 2
                    };
                    label = " ".repeat(left)
                        + &label
                        + &" ".repeat(difference - left);
                },
                Alignment::Right => {
                    label = " ".repeat(difference) + &label;
                },
            }
        }

        let mut style = Style::default();
        if self.selected {
            if self.enabled {
                style = style.add_modifier(Modifier::BOLD);
            }
        } else if !self.enabled {
            style = style.add_modifier(Modifier::DIM);
        };

        let block = Paragraph::new(label).style(style);
        frame.render_widget(block, area);
    }

    fn on_key(&mut self, ctx: &Context, key: KeyEvent) {
        if self.selected && self.enabled && KeyCode::Enter == key.code {
            (self.submit_fn)(ctx);
        }
    }

    fn on_tick(&mut self, _ctx: &Context) {}

    fn on_focus(&mut self, arrive: bool) {
        self.selected = arrive;
    }

    fn has_focus(&mut self) -> bool {
        self.selected
    }
}
