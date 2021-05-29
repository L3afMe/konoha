use std::io::Stdout;

use clap::crate_name;
use crossterm::event::{KeyCode, KeyModifiers};
use tui::{Frame, backend::CrosstermBackend, layout::{Alignment, Constraint, Direction, Layout, Rect}, widgets::{Block, Borders, Paragraph}};

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

// TODO: Tidy this
// Returns the rect in which the rest of the app can be drawn
pub fn draw_help_menu(
    frame: &mut CrosstermFrame,
    mut menu_help: Vec<(KeyModifiers, KeyCode, String)>,
    max_size: Rect,
) -> Rect {
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
    let seperator = ", ";
    let mut text = mapped.collect::<Vec<String>>().join(seperator);
    let mut split = split_text(&text, seperator, max_size.width as usize - 6);

    let mut lines = split.len() as u16;
    let mut longest = split
        .iter()
        .map(|line| line.len())
        .reduce(|l1, l2| l1.max(l2))
        .unwrap_or_else(|| text.len()) as u16;

    if longest + 4 > max_size.width {
        text = "Size too small to draw".to_string();
        split = split_text(&text, " ", max_size.width as usize - 4);

        lines = split.len() as u16;
        longest = split
            .iter()
            .map(|line| line.len())
            .reduce(|l1, l2| l1.max(l2))
            .unwrap_or_else(|| text.len()) as u16;
    }

    let layouts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(lines + 2)])
        .split(max_size);
    let top = layouts[0];
    let bottom = layouts[1];

    // Append 4 extra; 2 for padding and 2 for the border
    let layout = centered_line(longest + 4, lines + 2, 0, bottom);

    let help_block = Block::default().title("Help").borders(Borders::ALL);
    frame.render_widget(help_block, layout);

    for (idx, line) in split.iter().enumerate() {
        let layout = centered_line(longest, 1, idx as u16 + 1, bottom);
        let paragraph = Paragraph::new(line.as_ref()).alignment(Alignment::Center);
        frame.render_widget(paragraph, layout);
    }

    top
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

pub fn split_text(text: &str, sep: &str, max_size: usize) -> Vec<String> {
    let mut output = Vec::new();
    let mut input_remaining = text.to_string();

    while input_remaining.len() > max_size {
        let (split, remaining) =
            input_remaining.split_at(input_remaining.len().min(max_size));

        if split.contains(sep) {
            let (split, split_remaining) = split.rsplit_once(sep).unwrap();
            output.push(split.trim_matches(' ').to_string());
            input_remaining = (split_remaining.to_string() + remaining)
                .trim_matches(' ')
                .to_string();
        } else if remaining.starts_with(' ') || split.ends_with(' ') {
            output.push(split.trim_matches(' ').to_string());
            input_remaining = remaining.trim_matches(' ').to_string();
        }
    }

    output.push(input_remaining);

    output
}
