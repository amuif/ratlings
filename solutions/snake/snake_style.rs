use ratatui::{
    buffer::Buffer,
    style::{Color, Modifier, Style},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnakePart {
    Head,
    Body,
    Apple,
}

fn get_part_style(part: SnakePart) -> (char, Style) {
    match part {
        SnakePart::Head => (
            '@',
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ),
        SnakePart::Body => ('o', Style::default().fg(Color::Green)),
        SnakePart::Apple => (
            '*',
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
    }
}

fn render_part(x: u16, y: u16, part: SnakePart, buf: &mut Buffer) {
    let (symbol, style) = get_part_style(part);
    buf.set_string(x, y, symbol.to_string(), style);
}

fn main() {
    let mut buf = Buffer::empty(ratatui::layout::Rect::new(0, 0, 10, 1));
    render_part(0, 0, SnakePart::Head, &mut buf);
    render_part(1, 0, SnakePart::Body, &mut buf);
    render_part(2, 0, SnakePart::Apple, &mut buf);
    println!("{buf:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn test_head_style() {
        let (ch, style) = get_part_style(SnakePart::Head);
        assert_eq!(ch, '@');
        assert_eq!(style.fg, Some(Color::LightGreen));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_body_style() {
        let (ch, style) = get_part_style(SnakePart::Body);
        assert_eq!(ch, 'o');
        assert_eq!(style.fg, Some(Color::Green));
    }

    #[test]
    fn test_apple_style() {
        let (ch, style) = get_part_style(SnakePart::Apple);
        assert_eq!(ch, '*');
        assert_eq!(style.fg, Some(Color::Red));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_render_part() {
        let area = Rect::new(0, 0, 5, 1);
        let mut buf = Buffer::empty(area);
        render_part(1, 0, SnakePart::Head, &mut buf);

        let cell = buf.cell((1, 0)).unwrap();
        assert_eq!(cell.symbol(), "@");
        assert_eq!(cell.fg, Color::LightGreen);
        assert!(cell.modifier.contains(Modifier::BOLD));
    }
}
