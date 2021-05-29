use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};

use super::{Event, Menu};
use crate::app::{
    context::Context,
    helper::{centered_rect, CenterPosition, CrosstermFrame},
};

#[derive(Debug, Clone)]
pub struct LoadingMenu {
    text:     String,
    tick:     u16,
    progress: u16,
}

// Ticks per progress
static BAR_TICK_SPEED: u16 = 1;
static BAR_LENGTH: u16 = 20;

impl Menu for LoadingMenu {
    fn on_event(&mut self, event: Event, ctx: &Context) {
        if Event::Tick == event {
            self.on_tick(ctx)
        }
    }

    fn get_help_message(
        &mut self,
        _ctx: &Context,
    ) -> Vec<(KeyModifiers, KeyCode, String)> {
        vec![]
    }

    fn draw(
        &mut self,
        frame: &mut CrosstermFrame,
        max_size: Rect,
        _ctx: &Context,
    ) {
        let width = (BAR_LENGTH + 2).max(self.text.len() as u16);
        let text_height = self.text.split('\n').count() as u16;
        let height = 2 + text_height;

        let center = centered_rect(
            CenterPosition::AbsoluteInner(width, height),
            max_size,
        );
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(text_height),
                Constraint::Length(1), // Padding in the middle
                Constraint::Length(1),
            ])
            .split(center);

        let title =
            Paragraph::new(self.text.clone()).alignment(Alignment::Center);
        frame.render_widget(title, chunks[0]);

        let progress_bar_text = {
            let len = BAR_LENGTH as usize;
            let tick = self.progress as usize;
            if tick <= len {
                "█".repeat(tick) + &" ".repeat(len - tick)
            } else {
                let tick = tick - len;
                " ".repeat(tick) + &"█".repeat(len - tick)
            }
        };
        let progress_bar =
            Paragraph::new(progress_bar_text).alignment(Alignment::Center);
        frame.render_widget(progress_bar, chunks[2]);
    }

    fn get_minimum_size(&mut self) -> (u16, u16) {
        let split = self.text.split("\n").collect::<Vec<&str>>();
        let longest = split
            .iter()
            .map(|line| line.len())
            .reduce(|l1, l2| l1.max(l2))
            .unwrap_or_else(|| self.text.len()) as u16;

        let min_width = (BAR_LENGTH + 2).max(longest);
        let min_height = split.len() as u16;

        (min_width, min_height)
    }
}

impl LoadingMenu {
    pub fn new<T: ToString>(text: T) -> Self {
        Self {
            text:     text.to_string(),
            tick:     0,
            progress: 0,
        }
    }

    fn on_tick(&mut self, _ctx: &Context) {
        self.tick += 1;
        self.tick %= BAR_TICK_SPEED;
        if self.tick == 0 {
            self.progress += 1;
        }
        self.progress %= BAR_LENGTH * 2;
    }
}
