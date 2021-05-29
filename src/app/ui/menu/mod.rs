use std::ops::DerefMut;

use crossterm::event::{KeyCode, KeyModifiers};
use tui::layout::Rect;

use super::super::{context::Context, helper::CrosstermFrame};
use crate::app::event::Event;

pub mod authentication;
pub mod loading;

pub trait Menu {
    fn on_event(&mut self, event: Event, ctx: &Context);

    fn draw(
        &mut self,
        frame: &mut CrosstermFrame,
        max_size: Rect,
        ctx: &Context,
    );

    fn get_help_message(
        &mut self,
        ctx: &Context,
    ) -> Vec<(KeyModifiers, KeyCode, String)>;

    fn get_minimum_size(&mut self) -> (u16, u16);
}

impl Menu for Box<dyn Menu> {
    fn on_event(&mut self, event: Event, ctx: &Context) {
        self.deref_mut().on_event(event, ctx)
    }

    fn draw(
        &mut self,
        frame: &mut CrosstermFrame,
        max_size: Rect,
        ctx: &Context,
    ) {
        self.deref_mut().draw(frame, max_size, ctx)
    }

    fn get_help_message(
        &mut self,
        ctx: &Context,
    ) -> Vec<(KeyModifiers, KeyCode, String)> {
        self.deref_mut().get_help_message(ctx)
    }

    fn get_minimum_size(&mut self) -> (u16, u16) {
        self.deref_mut().get_minimum_size()
    }
}
