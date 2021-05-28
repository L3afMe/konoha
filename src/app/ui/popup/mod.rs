use crossterm::event::{KeyCode, KeyModifiers};
use tui::layout::{Constraint, Direction, Layout, Rect};

use super::prelude::Menu;
use crate::app::{
    context::Context,
    event::Event,
    helper::{centered_rect, CenterPosition, CrosstermFrame},
};

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
    menu: Box<dyn Menu + Send>,
    area: PopupArea,
}

impl Popup {
    pub fn get_area(&self, frame_size: Rect) -> Rect {
        match self.area {
            PopupArea::Dynamic(func) => func(frame_size),
            PopupArea::Absolute(width, height, pos) => {
                let max_size = centered_rect(
                    CenterPosition::AbsoluteOutter(2, 2),
                    frame_size,
                );

                match pos {
                    PopupPosition::TopLeft => {
                        let top = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(height),
                                Constraint::Min(0),
                            ])
                            .split(max_size)[0];

                        Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Length(width),
                                Constraint::Min(0),
                            ])
                            .split(top)[0]
                    },
                    PopupPosition::Top => {
                        let top = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(height),
                                Constraint::Min(0),
                            ])
                            .split(max_size)[0];

                        centered_rect(CenterPosition::AbsoluteInner(width, height), top)
                    },
                    PopupPosition::TopRight => {
                        let top = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(height),
                                Constraint::Min(0),
                            ])
                            .split(max_size)[0];

                        Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Min(0),
                                Constraint::Length(width),
                            ])
                            .split(top)[1]
                    },
                    PopupPosition::Left => {
                        let left = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Length(width),
                                Constraint::Min(0),
                            ])
                            .split(max_size)[0];

                        centered_rect(CenterPosition::AbsoluteInner(width, height), left)
                    },
                    PopupPosition::Center => centered_rect(
                        CenterPosition::AbsoluteInner(width, height),
                        max_size,
                    ),
                    PopupPosition::Right => {
                        let right = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Min(0),
                                Constraint::Length(width),
                            ])
                            .split(max_size)[1];

                        centered_rect(CenterPosition::AbsoluteInner(width, height), right)
                    },
                    PopupPosition::BottomLeft => {
                        let bottom = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Min(0),
                                Constraint::Length(height),
                            ])
                            .split(max_size)[1];

                        Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Length(width),
                                Constraint::Min(0),
                            ])
                            .split(bottom)[0]
                    },
                    PopupPosition::Bottom => {
                        let bottom = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Min(0),
                                Constraint::Length(height),
                            ])
                            .split(max_size)[1];

                        centered_rect(CenterPosition::AbsoluteInner(width, height), bottom)
                    },
                    PopupPosition::BottomRight => {
                        let bottom = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Min(0),
                                Constraint::Length(height),
                            ])
                            .split(max_size)[1];

                        Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Min(0),
                                Constraint::Length(width),
                            ])
                            .split(bottom)[1]
                    },
                }
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
