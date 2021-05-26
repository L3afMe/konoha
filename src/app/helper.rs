use std::io::Stdout;

use clap::crate_name;
use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub type CrosstermFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spacing {
    pub top:    u16,
    pub bottom: u16,
    pub left:   u16,
    pub right:  u16,
}

impl Spacing {
    pub fn new(top: u16, bottom: u16, left: u16, right: u16) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CenterPosition {
    Percentage(u16, u16),
    AbsoluteInner(u16, u16),
    AbsoluteOutter(u16, u16),
}

pub fn centered_rect(position: CenterPosition, base: Rect) -> Rect {
    let (constraints_x, constraints_y) = match position {
        CenterPosition::AbsoluteInner(x, y) => (
            [
                Constraint::Length((base.width - x) / 2),
                Constraint::Length(x),
                Constraint::Length((base.width - x) / 2),
            ],
            [
                Constraint::Length((base.height - y) / 2),
                Constraint::Length(y),
                Constraint::Length((base.height - y) / 2),
            ],
        ),
        CenterPosition::AbsoluteOutter(x, y) => (
            [
                Constraint::Length(x),
                Constraint::Length(base.width - (x * 2)),
                Constraint::Length(x),
            ],
            [
                Constraint::Length(y),
                Constraint::Length(base.height - (y * 2)),
                Constraint::Length(y),
            ],
        ),
        CenterPosition::Percentage(x, y) => (
            [
                Constraint::Percentage((100 - x) / 2),
                Constraint::Percentage(x),
                Constraint::Percentage((100 - x) / 2),
            ],
            [
                Constraint::Percentage((100 - y) / 2),
                Constraint::Percentage(y),
                Constraint::Percentage((100 - y) / 2),
            ],
        ),
    };
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints_y.as_ref())
        .split(base);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints_x.as_ref())
        .split(popup_layout[1])[1]
}

pub fn centered_line(
    width: u16,
    height: u16,
    top_padding: u16,
    base: Rect,
) -> Rect {
    let (constraints_x, constraints_y) = (
        [
            Constraint::Length((base.width - width) / 2),
            Constraint::Length(width),
            Constraint::Length((base.width - width) / 2),
        ],
        [
            Constraint::Length(top_padding),
            Constraint::Length(height),
            Constraint::Length(base.height - height - top_padding),
        ],
    );

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints_y.as_ref())
        .split(base);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints_x.as_ref())
        .split(popup_layout[1])[1]
}

pub fn split_rect(
    left_percentage: u16,
    direction: Direction,
    rect: Rect,
) -> [Rect; 2] {
    let split = Layout::default()
        .direction(direction)
        .constraints([
            Constraint::Percentage(left_percentage),
            Constraint::Percentage(100 - left_percentage),
        ])
        .split(rect);

    [split[0], split[1]]
}

pub fn expand_area(mut area: Rect, spacing: Spacing) -> Rect {
    area.x -= spacing.left;
    area.y -= spacing.top;
    area.width += spacing.left + spacing.right;
    area.height += spacing.top + spacing.bottom;

    area
}

pub fn shrink_area(mut area: Rect, spacing: Spacing) -> Rect {
    area.x += spacing.left;
    area.y += spacing.top;
    area.width -= spacing.left + spacing.right;
    area.height -= spacing.top + spacing.bottom;

    area
}

pub fn draw_help_menu(
    frame: &mut CrosstermFrame,
    mut menu_help: Vec<(KeyModifiers, KeyCode, String)>,
    bottom_layout: Rect,
) {
    menu_help.insert(
        0,
        (
            KeyModifiers::ALT,
            KeyCode::Char('?'),
            "Toggle help menu".to_string(),
        ),
    );
    menu_help.insert(
        1,
        (
            KeyModifiers::CONTROL,
            KeyCode::Char('d'),
            format!("Exit {}", crate_name!()),
        ),
    );
    let mapped = menu_help.into_iter().map(|(mods, key, msg)| {
        let mut mod_str = String::new();
        if mods.contains(KeyModifiers::ALT) {
            mod_str += "Alt+";
        }
        if mods.contains(KeyModifiers::CONTROL) {
            mod_str += "Ctrl+";
        }
        if mods.contains(KeyModifiers::SHIFT) {
            mod_str += "Shft+";
        }

        mod_str += format!("{} - {}", keycode_to_str(key), msg).as_ref();

        mod_str
    });
    let text = mapped.collect::<Vec<String>>().join(", ");

    let layout = centered_line((text.len() + 4) as u16, 3, 0, bottom_layout);
    // Border adding size messing with shit

    let help_block = Block::default().title("Help").borders(Borders::ALL);
    frame.render_widget(help_block, layout);

    let layout = centered_line(text.len() as u16, 1, 1, bottom_layout);
    let paragraph = Paragraph::new(text.as_ref());
    frame.render_widget(paragraph, layout)
}

fn keycode_to_str(key: KeyCode) -> String {
    match key {
        KeyCode::F(x) => format!("F{}", x),
        KeyCode::Char(x) => x.to_string().to_uppercase(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Escape".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::PageUp => "Page Up".to_string(),
        KeyCode::PageDown => "Page Down".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "Backtab".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Null => String::default(),
    }
}
