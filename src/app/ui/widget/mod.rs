use std::ops::DerefMut;

use crossterm::event::KeyEvent;
use tui::layout::Rect;

use crate::app::{context::Context, helper::CrosstermFrame};

pub mod button;
pub mod input;

pub trait Widget {
    fn on_key(&mut self, ctx: &Context, key: KeyEvent);
    fn on_tick(&mut self, ctx: &Context);

    fn render(&mut self, area: Rect, frame: &mut CrosstermFrame);
    fn has_focus(&mut self) -> bool;
    fn on_focus(&mut self, arrive: bool);
}

impl Widget for Box<dyn Widget> {
    fn render(&mut self, area: Rect, frame: &mut CrosstermFrame) {
        self.deref_mut().render(area, frame)
    }

    fn on_key(&mut self, ctx: &Context, key: KeyEvent) {
        self.deref_mut().on_key(ctx, key)
    }

    fn on_tick(&mut self, ctx: &Context) {
        self.deref_mut().on_tick(ctx)
    }

    fn has_focus(&mut self) -> bool {
        self.deref_mut().has_focus()
    }

    fn on_focus(&mut self, arrive: bool) {
        self.deref_mut().on_focus(arrive)
    }
}
