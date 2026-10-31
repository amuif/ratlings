use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget, Wrap},
};

fn render_message(text: &str, area: Rect, buf: &mut Buffer) {
    Paragraph::new(text)
        .block(Block::bordered().title(" Message "))
        .wrap(Wrap { trim: true })
        .centered()
        .render(area, buf);
}

fn main() {
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
