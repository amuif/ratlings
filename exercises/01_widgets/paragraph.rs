// `Paragraph` displays text. It can align the text, wrap long lines and draw
// itself inside a `Block`.
//
// Complete `render_message` so that it renders `text` as a `Paragraph` that:
// 1. Is wrapped in a bordered `Block` titled " Message ".
// 2. Wraps long lines, trimming leading whitespace.
// 3. Is centered horizontally.
//
// Execute `rustlings hint paragraph` or use the `hint` watch subcommand for a hint.

#![allow(unused_imports, unused_variables)]

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget, Wrap},
};

fn render_message(text: &str, area: Rect, buf: &mut Buffer) {
    // TODO: Render `text` as a wrapped, centered `Paragraph` inside a `Block`.
    todo!()
}

fn main() {
    // You can optionally experiment here.
    let area = Rect::new(0, 0, 16, 5);
    let mut buf = Buffer::empty(area);
    render_message("Hello from Ratatui widgets", area, &mut buf);
    println!("{buf:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_short_message() {
        let area = Rect::new(0, 0, 16, 3);
        let mut buf = Buffer::empty(area);
        render_message("Hi!", area, &mut buf);

        let expected =
            Buffer::with_lines(["┌ Message ─────┐", "│      Hi!     │", "└──────────────┘"]);
        assert_eq!(buf, expected);
    }

    #[test]
    fn test_render_long_message_wraps() {
        let area = Rect::new(0, 0, 16, 5);
        let mut buf = Buffer::empty(area);
        render_message("Hello from Ratatui widgets", area, &mut buf);

        let expected = Buffer::with_lines([
            "┌ Message ─────┐",
            "│  Hello from  │",
            "│    Ratatui   │",
            "│    widgets   │",
            "└──────────────┘",
        ]);
        assert_eq!(buf, expected);
    }
}
