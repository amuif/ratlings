// `Layout` splits an area into smaller areas. Each `Constraint` describes how
// big one of the areas should be.
//
// Complete `app_layout` so that it splits `area` vertically into:
// 1. A header that is exactly 1 row tall.
// 2. A body that fills all of the remaining space.
// 3. A footer that is exactly 1 row tall.
//
// Execute `rustlings hint layout` or use the `hint` watch subcommand for a hint.

#![allow(unused_imports, unused_variables)]

use ratatui::layout::{Constraint, Layout, Rect};

fn app_layout(area: Rect) -> [Rect; 3] {
    // TODO: Split `area` into a header, a body and a footer.
    todo!()
}

fn main() {
    // You can optionally experiment here.
    let [header, body, footer] = app_layout(Rect::new(0, 0, 40, 10));
    println!("header: {header}\nbody: {body}\nfooter: {footer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_layout() {
        let [header, body, footer] = app_layout(Rect::new(0, 0, 40, 10));
        assert_eq!(header, Rect::new(0, 0, 40, 1));
        assert_eq!(body, Rect::new(0, 1, 40, 8));
        assert_eq!(footer, Rect::new(0, 9, 40, 1));
    }

    #[test]
    fn test_body_grows_with_area() {
        let [header, body, footer] = app_layout(Rect::new(5, 2, 80, 30));
        assert_eq!(header, Rect::new(5, 2, 80, 1));
        assert_eq!(body, Rect::new(5, 3, 80, 28));
        assert_eq!(footer, Rect::new(5, 31, 80, 1));
    }
}
