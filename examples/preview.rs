//! A playable preview of the Snake game you'll build in Ratlings.
//!
//! Steer with the vim keys (`h` `j` `k` `l`) or the arrow keys.
//!
//! Run it with `cargo run --example preview`.

use std::{
    collections::VecDeque,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Flex, Layout, Margin, Position, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};

/// The time for one vertical move. Terminal cells are about twice as tall as
/// they are wide, so horizontal moves take half as long to look as fast.
const START_TICK: Duration = Duration::from_millis(150);
const FASTEST_TICK: Duration = Duration::from_millis(50);
/// How much faster the snake gets for every apple it eats.
const SPEED_UP: Duration = Duration::from_millis(6);
const START_LEN: u16 = 4;
/// The snake grows one cell for every this many points.
const POINTS_PER_CELL: u32 = 2;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut rng = Rng::seeded();
    let mut game: Option<Game> = None;
    let mut last_step = Instant::now();
    loop {
        terminal.draw(|frame| draw(frame, &mut game, &mut rng))?;

        // Wait for a key until the next move is due, so pressing keys quickly
        // never makes the snake move faster.
        let tick = game.as_ref().map_or(START_TICK, Game::tick);
        if event::poll(tick.saturating_sub(last_step.elapsed()))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('h') | KeyCode::Left => turn(&mut game, Direction::Left),
                KeyCode::Char('j') | KeyCode::Down => turn(&mut game, Direction::Down),
                KeyCode::Char('k') | KeyCode::Up => turn(&mut game, Direction::Up),
                KeyCode::Char('l') | KeyCode::Right => turn(&mut game, Direction::Right),
                KeyCode::Char('p') => {
                    if let Some(game) = &mut game {
                        game.toggle_pause();
                    }
                }
                KeyCode::Char('r') | KeyCode::Enter => {
                    if let Some(old) = &game
                        && old.state == State::GameOver
                    {
                        game = Some(Game::new(old.bounds, &mut rng));
                    }
                }
                _ => {}
            }
        }

        if last_step.elapsed() >= tick {
            if let Some(game) = &mut game {
                game.step(&mut rng);
            }
            last_step = Instant::now();
        }
    }
}

fn turn(game: &mut Option<Game>, direction: Direction) {
    if let Some(game) = game {
        game.turn(direction);
    }
}

// The three functions below are the solutions to the Ratlings exercises, so the
// game you build looks exactly like this preview.

/// Exercise `snake_arena`: the arena is a bordered, cyan `Block` with titles.
fn render_arena(area: Rect, score: u32, length: usize, buf: &mut Buffer) {
    let block = Block::bordered()
        .title(format!(" Snake — Score: {score} "))
        .title_top(Line::from(format!(" Length: {length} ")).right_aligned())
        .border_style(Style::new().fg(Color::Cyan));
    block.render(area, buf);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnakePart {
    Head,
    Body,
    Apple,
}

/// Exercise `snake_style`: each part of the game gets its own symbol and style.
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

struct GameLayout {
    header: Rect,
    arena: Rect,
    footer: Rect,
}

/// Exercise `snake_layout`: split the screen into a header, arena and footer.
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

fn draw(frame: &mut Frame, game: &mut Option<Game>, rng: &mut Rng) {
    let GameLayout {
        header,
        arena,
        footer,
    } = create_game_layout(frame.area());

    let title =
        Paragraph::new(Line::from(" 🐀 Welcome to Ratlings! Here's what you'll build ").centered())
            .style(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::bordered());
    frame.render_widget(title, header);

    let (score, length) = game
        .as_ref()
        .map_or((0, 0), |game| (game.score, game.snake.len()));
    render_arena(arena, score, length, frame.buffer_mut());
    let inner = arena.inner(Margin::new(1, 1));

    if inner.width < START_LEN * 2 || inner.height < 3 {
        *game = None;
        frame.render_widget(
            Paragraph::new("Make your terminal a little bigger 🙂"),
            inner,
        );
    } else {
        if game.as_ref().is_none_or(|game| game.bounds != inner) {
            *game = Some(Game::new(inner, rng));
        }
        if let Some(game) = game {
            game.render(frame.buffer_mut());
            if let Some(message) = game.message() {
                render_message(frame, inner, &message);
            }
        }
    }

    let help = Paragraph::new(vec![
        Line::from(" Move: h j k l / arrows   •   Pause: p   •   Restart: r   •   Quit: q / Esc"),
        Line::from(
            " Build it yourself: 1. snake_arena (Block) → 2. snake_style (Style) → 3. snake_layout (Layout)",
        ),
    ])
    .style(Style::new().fg(Color::DarkGray))
    .block(Block::bordered().title(" Controls "));
    frame.render_widget(help, footer);
}

fn render_message(frame: &mut Frame, area: Rect, message: &str) {
    let width = u16::try_from(message.chars().count() + 4).unwrap_or(u16::MAX);
    let [row] = Layout::vertical([Constraint::Length(3)])
        .flex(Flex::Center)
        .areas(area);
    let [popup] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(row);
    let paragraph = Paragraph::new(message)
        .centered()
        .style(Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::bordered());
    frame.render_widget(Clear, popup);
    frame.render_widget(paragraph, popup);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// Waiting for the first key press.
    Ready,
    Playing,
    Paused,
    GameOver,
}

struct Game {
    bounds: Rect,
    state: State,
    /// The front is the head.
    snake: VecDeque<Position>,
    /// The direction of the last move.
    direction: Direction,
    /// The direction of the next move, set by the player.
    next_direction: Direction,
    apple: Position,
    score: u32,
    /// Cells still to grow from apples already eaten.
    growth: u16,
}

impl Game {
    fn new(bounds: Rect, rng: &mut Rng) -> Self {
        let y = bounds.y + bounds.height / 2;
        let snake = (0..START_LEN)
            .rev()
            .map(|offset| Position::new(bounds.x + offset, y))
            .collect();
        let mut game = Self {
            bounds,
            state: State::Ready,
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            apple: Position::new(bounds.x, bounds.y),
            score: 0,
            growth: 0,
        };
        game.apple = game.free_position(rng);
        game
    }

    /// The time between moves: every apple eaten makes the snake faster.
    fn tick(&self) -> Duration {
        let tick = START_TICK
            .saturating_sub(SPEED_UP * self.score)
            .max(FASTEST_TICK);
        match self.next_direction {
            Direction::Up | Direction::Down => tick,
            Direction::Left | Direction::Right => tick / 2,
        }
    }

    fn turn(&mut self, direction: Direction) {
        match self.state {
            State::Ready => self.state = State::Playing,
            State::Playing => {}
            State::Paused | State::GameOver => return,
        }
        // A snake can't turn back on itself.
        if direction != self.direction.opposite() {
            self.next_direction = direction;
        }
    }

    fn toggle_pause(&mut self) {
        self.state = match self.state {
            State::Playing => State::Paused,
            State::Paused => State::Playing,
            state => state,
        };
    }

    fn message(&self) -> Option<String> {
        match self.state {
            State::Ready => Some("Press h j k l or an arrow key to start".into()),
            State::Playing => None,
            State::Paused => Some("Paused: press p to continue".into()),
            State::GameOver => Some(format!(
                "Game over! Score: {}. Press r to play again",
                self.score
            )),
        }
    }

    /// Moves the snake one cell, growing when it eats the apple and ending the
    /// game when it hits a wall or itself.
    fn step(&mut self, rng: &mut Rng) {
        if self.state != State::Playing {
            return;
        }
        self.direction = self.next_direction;

        let head = self.snake[0];
        let next = match self.direction {
            Direction::Up if head.y > self.bounds.top() => Position::new(head.x, head.y - 1),
            Direction::Down if head.y + 1 < self.bounds.bottom() => {
                Position::new(head.x, head.y + 1)
            }
            Direction::Left if head.x > self.bounds.left() => Position::new(head.x - 1, head.y),
            Direction::Right if head.x + 1 < self.bounds.right() => {
                Position::new(head.x + 1, head.y)
            }
            _ => {
                self.state = State::GameOver;
                return;
            }
        };

        // Leaving the tail in place is what makes the snake longer.
        if self.growth > 0 {
            self.growth -= 1;
        } else {
            self.snake.pop_back();
        }
        if self.snake.contains(&next) {
            self.state = State::GameOver;
            return;
        }
        self.snake.push_front(next);

        if next == self.apple {
            self.score += 1;
            if self.score.is_multiple_of(POINTS_PER_CELL) {
                self.growth += 1;
            }
            self.apple = self.free_position(rng);
        }
    }

    fn free_position(&self, rng: &mut Rng) -> Position {
        let mut position = self.bounds.as_position();
        for _ in 0..100 {
            position = Position::new(
                self.bounds.x + rng.below(self.bounds.width),
                self.bounds.y + rng.below(self.bounds.height),
            );
            if !self.snake.contains(&position) {
                break;
            }
        }
        position
    }

    fn render(&self, buf: &mut Buffer) {
        render_part(self.apple.x, self.apple.y, SnakePart::Apple, buf);
        for part in self.snake.iter().skip(1) {
            render_part(part.x, part.y, SnakePart::Body, buf);
        }
        render_part(self.snake[0].x, self.snake[0].y, SnakePart::Head, buf);
    }
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

    fn below(&mut self, n: u16) -> u16 {
        u16::try_from(self.next_u64() % u64::from(n.max(1))).unwrap_or(0)
    }
}
