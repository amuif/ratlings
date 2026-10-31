use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{List, ListState, StatefulWidget},
};

fn render_menu(items: &[&str], selected: usize, area: Rect, buf: &mut Buffer) {
    let list = List::new(items.iter().copied())
        .highlight_symbol("> ")
        .highlight_style(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    let mut state = ListState::default().with_selected(Some(selected));
    StatefulWidget::render(list, area, buf, &mut state);
}

fn main() {
    let area = Rect::new(0, 0, 12, 3);
    let mut buf = Buffer::empty(area);
    render_menu(&["New game", "Settings", "Quit"], 1, area, &mut buf);
    println!("{buf:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const ITEMS: [&str; 3] = ["New game", "Settings", "Quit"];

    #[test]
    fn test_first_item_selected() {
        let area = Rect::new(0, 0, 12, 3);
        let mut buf = Buffer::empty(area);
        render_menu(&ITEMS, 0, area, &mut buf);

        let mut expected = Buffer::with_lines([
            "> New game  ", //
            "  Settings  ",
            "  Quit      ",
        ]);
        expected.set_style(
            Rect::new(0, 0, 12, 1),
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        );
        assert_eq!(buf, expected);
    }

    #[test]
    fn test_last_item_selected() {
        let area = Rect::new(0, 0, 12, 3);
        let mut buf = Buffer::empty(area);
        render_menu(&ITEMS, 2, area, &mut buf);

        let mut expected = Buffer::with_lines([
            "  New game  ", //
            "  Settings  ",
            "> Quit      ",
        ]);
        expected.set_style(
            Rect::new(0, 2, 12, 1),
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        );
        assert_eq!(buf, expected);
    }
}
