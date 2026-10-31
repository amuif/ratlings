// `List` displays a column of items and can highlight one of them. The
// selection is not part of the widget itself. It lives in a `ListState`, which
// makes `List` a `StatefulWidget`.
//
// Complete `render_menu` so that it renders `items` as a `List` that:
// 1. Marks the selected item with the symbol "> ".
// 2. Highlights the selected item in bold yellow.
// 3. Selects the item at index `selected`.
//
// Execute `rustlings hint list` or use the `hint` watch subcommand for a hint.

#![allow(unused_imports, unused_variables)]

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{List, ListState, StatefulWidget},
};

fn render_menu(items: &[&str], selected: usize, area: Rect, buf: &mut Buffer) {
    // TODO: Create a `List` and a `ListState`, then render them together.
    todo!()
}

fn main() {
    // You can optionally experiment here.
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
