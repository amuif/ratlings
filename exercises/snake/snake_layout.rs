// In Ratatui, the terminal screen is arranged into responsive regions using `Layout`.
// `Layout` splits a given `Rect` according to a direction and a list of `Constraint`s.
//
// In this final exercise of our Snake mini-series, we build the full game screen layout!
//
// Complete the `create_game_layout` function:
// Use `Layout::vertical(...)` with three constraints:
// 1. `Constraint::Length(3)`: Header for the game title.
// 2. `Constraint::Min(0)`: The main arena, occupying the remaining space.
// 3. `Constraint::Length(4)`: Footer for the keyboard controls (two lines inside a border).
//
// Return the partitioned areas in a `GameLayout` struct.
//
// Execute `rustlings run snake_layout` or use `hint` for help.

#![allow(unused_imports, unused_variables)]

use ratatui::layout::{Constraint, Layout, Rect};

#[derive(Debug, PartialEq, Eq)]
pub struct GameLayout {
    pub header: Rect,
    pub arena: Rect,
    pub footer: Rect,
}

fn create_game_layout(area: Rect) -> GameLayout {
    // TODO: Split `area` vertically using Layout and the specified constraints:
    // - Header: Constraint::Length(3)
    // - Arena: Constraint::Min(0)
    // - Footer: Constraint::Length(4)
    todo!()
}

fn main() {
    // You can optionally experiment with your code here.
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
