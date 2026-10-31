use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

fn render_panel(area: Rect, buf: &mut Buffer) {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Inbox ")
        .title_bottom(Line::from(" 3 unread ").right_aligned())
        .render(area, buf);
}

fn main() {
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
