// `Block` is the most common Ratatui widget. It draws borders and titles around
// an area, and other widgets are usually wrapped in one.
//
// Complete `render_panel` so that it renders a `Block` that:
// 1. Has borders on all sides, with rounded corners.
// 2. Has the title " Inbox " on the top border.
// 3. Has the right-aligned title " 3 unread " on the bottom border.
//
// Execute `rustlings hint block` or use the `hint` watch subcommand for a hint.

#![allow(unused_imports, unused_variables)]

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

fn render_panel(area: Rect, buf: &mut Buffer) {
    // TODO: Render a rounded, bordered `Block` with the two titles.
    todo!()
}

fn main() {
    // You can optionally experiment here.
    let area = Rect::new(0, 0, 20, 4);
    let mut buf = Buffer::empty(area);
    render_panel(area, &mut buf);
    println!("{buf:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_panel() {
        let area = Rect::new(0, 0, 20, 4);
        let mut buf = Buffer::empty(area);
        render_panel(area, &mut buf);

        let expected = Buffer::with_lines([
            "╭ Inbox ───────────╮",
            "│                  │",
            "│                  │",
            "╰──────── 3 unread ╯",
        ]);
        assert_eq!(buf, expected);
    }

    #[test]
    fn test_render_panel_fills_any_area() {
        let area = Rect::new(0, 0, 12, 3);
        let mut buf = Buffer::empty(area);
        render_panel(area, &mut buf);

        let expected = Buffer::with_lines(["╭ Inbox ───╮", "│          │", "╰ 3 unread ╯"]);
        assert_eq!(buf, expected);
    }
}
