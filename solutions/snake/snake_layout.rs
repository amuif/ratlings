use ratatui::layout::{Constraint, Layout, Rect};

#[derive(Debug, PartialEq, Eq)]
pub struct GameLayout {
    pub header: Rect,
    pub arena: Rect,
    pub footer: Rect,
}

fn create_game_layout(area: Rect) -> GameLayout {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(4),
    ])
    .split(area);

    GameLayout {
        header: chunks[0],
        arena: chunks[1],
        footer: chunks[2],
    }
}

fn main() {
    let full_screen = Rect::new(0, 0, 80, 24);
    let layout = create_game_layout(full_screen);
    println!("{layout:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_layout_dimensions() {
        let screen = Rect::new(0, 0, 80, 24);
        let layout = create_game_layout(screen);

        assert_eq!(layout.header, Rect::new(0, 0, 80, 3));
        assert_eq!(layout.arena, Rect::new(0, 3, 80, 17));
        assert_eq!(layout.footer, Rect::new(0, 20, 80, 4));
    }

    #[test]
    fn test_game_layout_small_screen() {
        let screen = Rect::new(0, 0, 40, 10);
        let layout = create_game_layout(screen);

        assert_eq!(layout.header, Rect::new(0, 0, 40, 3));
        assert_eq!(layout.arena, Rect::new(0, 3, 40, 3));
        assert_eq!(layout.footer, Rect::new(0, 6, 40, 4));
    }
}
