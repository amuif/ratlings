// In Ratatui, visual containers and borders are represented by the `Block` widget.
// A `Block` can draw borders on any side, color them, and render titles (such as
// the game score) in different positions.
//
// In this first exercise of our Snake mini-series, we build the playing arena!
//
// Complete the `render_arena` function:
// 1. Create a `Block` with all borders enabled (hint: `Block::bordered()`).
// 2. Set its title to `" Snake — Score: {score} "` where `{score}` is the passed
//    score argument.
// 3. Add a second, right-aligned title `" Length: {length} "` on the top border
//    (hint: `.title_top(...)` with a right-aligned `Line`).
// 4. Color the border cyan (hint: `.border_style(...)`).
// 5. Render the block into the given `buf` inside `area`.
//
// Execute `rustlings run snake_arena` or use `hint` for help.

#![allow(unused_imports, unused_variables)]

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Widget},
};

fn render_arena(area: Rect, score: u32, length: usize, buf: &mut Buffer) {
    // TODO: Create a bordered, cyan Block with the title " Snake — Score: {score} "
    // and a right-aligned " Length: {length} " title, then render it into `buf`.
    todo!()
}

fn main() {
    // You can optionally experiment with your code here.
    let area = Rect::new(0, 0, 40, 8);
    let mut buf = Buffer::empty(area);
    render_arena(area, 10, 9, &mut buf);
    println!("{buf:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected_arena(area: Rect, score: u32, length: usize) -> Buffer {
        let mut expected = Buffer::empty(area);
        Block::bordered()
            .title(format!(" Snake — Score: {score} "))
            .title_top(Line::from(format!(" Length: {length} ")).right_aligned())
            .border_style(Style::new().fg(Color::Cyan))
            .render(area, &mut expected);
        expected
    }

    #[test]
    fn test_render_arena_zero_score() {
        let area = Rect::new(0, 0, 40, 10);
        let mut buf = Buffer::empty(area);
        render_arena(area, 0, 4, &mut buf);

        assert_eq!(buf, expected_arena(area, 0, 4));
    }

    #[test]
    fn test_render_arena_with_score() {
        let area = Rect::new(0, 0, 40, 10);
        let mut buf = Buffer::empty(area);
        render_arena(area, 42, 25, &mut buf);

        assert_eq!(buf, expected_arena(area, 42, 25));
    }

    #[test]
    fn test_render_arena_border_is_cyan() {
        let area = Rect::new(0, 0, 40, 10);
        let mut buf = Buffer::empty(area);
        render_arena(area, 0, 4, &mut buf);

        assert_eq!(buf.cell((0, 5)).unwrap().fg, Color::Cyan);
        assert_eq!(buf.cell((39, 9)).unwrap().fg, Color::Cyan);
    }
}
