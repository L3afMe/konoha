use crossterm::event::{KeyCode, KeyModifiers};
use tui::layout::Rect;

use super::prelude::Menu;
use crate::app::{context::Context, event::Event, helper::{centered_rect, CenterPosition, CrosstermFrame}};

pub mod confirmation;
pub mod message;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PopupPosition {
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Default for PopupPosition {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PopupArea {
    // The absolute x, y and position
    Absolute(u16, u16, PopupPosition),
    // Takes in the maximum size (usually screen
    //   size minus the help menu) and return the
    //   desired size
    Dynamic(fn(Rect) -> Rect),
    //
}

impl Default for PopupArea {
    fn default() -> Self {
        Self::Dynamic(|area| {
            centered_rect(CenterPosition::Percentage(60, 40), area)
        })
    }
}

pub struct Popup {
    menu:       Box<dyn Menu + Send>,
    area:       PopupArea,
}

impl Popup {
    pub fn get_area(&self, frame_size: Rect) -> Rect {
        match self.area {
            PopupArea::Dynamic(func) => func(frame_size),
            PopupArea::Absolute(widget, height, pos) => match pos {
                // TODO: Other positions
                _ => centered_rect(
                    CenterPosition::AbsoluteInner(widget, height),
                    frame_size,
                ),
            },
        }
    }
}

impl Menu for Popup {
    fn draw(
        &mut self,
        frame: &mut CrosstermFrame,
        max_size: Rect,
        ctx: &Context,
    ) {
        self.menu.draw(frame, max_size, ctx)
    }

    fn on_event(&mut self, event: Event, ctx: &Context) {
        self.menu.on_event(event, ctx)
    }

    fn get_help_message(
        &mut self,
        ctx: &Context,
    ) -> Vec<(KeyModifiers, KeyCode, String)> {
        self.menu.get_help_message(ctx)
    }
}
