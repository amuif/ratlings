//! The celebration for finishing every Ratlings exercise: confetti!
//!
//! Run it with `cargo run --example finale`.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
};

const TICK: Duration = Duration::from_millis(40);
const GRAVITY: f32 = 0.05;
const SYMBOLS: [&str; 6] = ["*", "+", "•", "◆", "▲", "●"];
const COLORS: [Color; 7] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
    Color::LightRed,
];

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    symbol: &'static str,
    color: Color,
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut rng = Rng::seeded();
    let mut particles: Vec<Particle> = Vec::new();
    let mut tick: usize = 0;
    loop {
        let size = terminal.size()?;
        let (width, height) = (f32::from(size.width), f32::from(size.height));

        // Fire both confetti cannons from the bottom corners every so often,
        // and keep a light sprinkle falling from the top in between.
        if tick.is_multiple_of(25) {
            for _ in 0..60 {
                let left = rng.unit() < 0.5;
                let speed = (2.0 * GRAVITY * height).sqrt() * (0.7 + 0.3 * rng.unit());
                let drift = (0.3 + rng.unit()) * (width / 80.0).max(0.5);
                particles.push(Particle {
                    x: if left { 0.0 } else { width - 1.0 },
                    y: height - 1.0,
                    vx: if left { drift } else { -drift },
                    vy: -speed,
                    symbol: rng.pick(&SYMBOLS),
                    color: rng.pick(&COLORS),
                });
            }
        }
        for _ in 0..2 {
            particles.push(Particle {
                x: rng.unit() * width,
                y: 0.0,
                vx: rng.unit() * 0.4 - 0.2,
                vy: 0.0,
                symbol: rng.pick(&SYMBOLS),
                color: rng.pick(&COLORS),
            });
        }

        for particle in &mut particles {
            particle.x += particle.vx;
            particle.y += particle.vy;
            particle.vy = (particle.vy + GRAVITY).min(0.6);
            particle.vx *= 0.98;
        }
        particles.retain(|p| p.y < height && p.x >= 0.0 && p.x < width);

        terminal.draw(|frame| draw(frame, &particles, tick))?;

        if event::poll(TICK)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter)
        {
            return Ok(());
        }
        tick = tick.wrapping_add(1);
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn draw(frame: &mut Frame, particles: &[Particle], tick: usize) {
    let area = frame.area();
    let buf = frame.buffer_mut();
    for particle in particles {
        if particle.x < 0.0 || particle.y < 0.0 {
            continue;
        }
        let (x, y) = (particle.x as u16, particle.y as u16);
        if x < area.width && y < area.height {
            buf.set_string(
                area.x + x,
                area.y + y,
                particle.symbol,
                Style::new().fg(particle.color),
            );
        }
    }

    let popup = centered(area, 48, 9);
    let flashing = COLORS[(tick / 6) % COLORS.len()];
    let bold = Style::new().add_modifier(Modifier::BOLD);
    let message = Paragraph::new(vec![
        Line::from(Span::styled("🎉 Congratulations! 🎉", bold.fg(flashing))),
        Line::from(""),
        Line::from("You have finished the Ratlings Snake game!"),
        Line::from("Block ✓   Style ✓   Layout ✓"),
        Line::from(vec![
            Span::styled("oooo", Style::new().fg(Color::Green)),
            Span::styled("@", bold.fg(Color::LightGreen)),
            Span::raw("  "),
            Span::styled("*", bold.fg(Color::Red)),
        ]),
        Line::from(Span::styled(
            "Press q to exit",
            Style::new().fg(Color::DarkGray),
        )),
    ])
    .centered()
    .block(
        Block::bordered()
            .title(" 🐀 Ratlings ")
            .border_style(Style::new().fg(flashing)),
    );
    frame.render_widget(Clear, popup);
    frame.render_widget(message, popup);
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [row] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [popup] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(row);
    popup
}

/// A tiny xorshift generator, so the example doesn't need extra dependencies.
struct Rng(u64);

impl Rng {
    fn seeded() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.subsec_nanos());
        Self(u64::from(nanos) | 1)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A random number in `0.0..=1.0`.
    fn unit(&mut self) -> f32 {
        f32::from(u16::try_from(self.next_u64() >> 48).unwrap_or(0)) / f32::from(u16::MAX)
    }

    fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        let index = usize::try_from(self.next_u64() % items.len() as u64).unwrap_or(0);
        items[index]
    }
}
