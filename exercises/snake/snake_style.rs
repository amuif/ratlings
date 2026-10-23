// In Ratatui, visual elements are styled using `Style`, `Color`, and `Modifier`.
//
// In this second exercise of our Snake mini-series, we give our snake and apple
// distinct symbols and colors!
//
// Complete the `get_part_style` function:
// 1. `SnakePart::Head`:
//    - Symbol: '@'
//    - Style: foreground `Color::LightGreen`, modifier `Modifier::BOLD`
// 2. `SnakePart::Body`:
//    - Symbol: 'o'
//    - Style: foreground `Color::Green`
// 3. `SnakePart::Apple`:
//    - Symbol: '*'
//    - Style: foreground `Color::Red`, modifier `Modifier::BOLD`
//
// Execute `rustlings run snake_style` or use `hint` for help.

#![allow(unused_imports, unused_variables)]

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
    // TODO: Match on `part` and return the specified (char, Style) tuple
    todo!()
}

fn render_part(x: u16, y: u16, part: SnakePart, buf: &mut Buffer) {
    let (symbol, style) = get_part_style(part);
    buf.set_string(x, y, symbol.to_string(), style);
}

fn main() {
    // You can optionally experiment with your code here.
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
