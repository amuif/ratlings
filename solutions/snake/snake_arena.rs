use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Widget},
};

fn render_arena(area: Rect, score: u32, length: usize, buf: &mut Buffer) {
    let block = Block::bordered()
        .title(format!(" Snake — Score: {score} "))
        .title_top(Line::from(format!(" Length: {length} ")).right_aligned())
        .border_style(Style::new().fg(Color::Cyan));
    block.render(area, buf);
}

fn main() {
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
