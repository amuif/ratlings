use ratatui::layout::{Constraint, Layout, Rect};

fn app_layout(area: Rect) -> [Rect; 3] {
    Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(area)
}

fn main() {
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
