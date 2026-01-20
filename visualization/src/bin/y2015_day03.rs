use std::{
    fmt::Display,
    io::{self},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use euclid::default::{Point2D, Translation2D};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1);

    ratatui::run(|terminal| {
        let mut app = App::new(input_path)?;
        app.run(terminal)
    })
}

#[derive(Debug, Default)]
pub struct App {
    santa_pos: Point2D<i32>,
    houses: Vec<Point2D<i32>>,
    input: Vec<char>,
    input_path: String,
    position: usize,
    current_delay: f32,
    target_delay: f32,
    now: u128,
    state: AppState,
}

#[derive(Debug, Default, PartialEq)]
enum AppState {
    #[default]
    Idle,
    Running,
    Exit,
}

impl Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Running => write!(f, "Running"),
            Self::Exit => write!(f, "Exit"),
        }
    }
}

impl App {
    pub fn new(input_path: Option<String>) -> io::Result<Self> {
        let (input, input_path) = if let Some(path) = input_path {
            (std::fs::read_to_string(&path)?, path)
        } else {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            (input, String::from("stdin"))
        };

        Ok(Self {
            input: input.chars().collect(),
            input_path,
            ..Default::default()
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state != AppState::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => self.toggle_run_visualization(),
            KeyCode::Char(' ') => self.tick_visualization(0.0, true),
            KeyCode::Up => self.change_delay(-10.0),
            KeyCode::Down => self.change_delay(10.0),
            _ => {}
        }
    }

    const fn exit(&mut self) {
        self.state = AppState::Exit;
    }

    const fn toggle_run_visualization(&mut self) {
        self.state = match self.state {
            AppState::Idle => AppState::Running,
            AppState::Running => AppState::Idle,
            AppState::Exit => AppState::Exit,
        };
    }

    const fn change_delay(&mut self, delta: f32) {
        self.target_delay = (self.target_delay + delta).clamp(0.0, 10000.0);
    }

    fn tick_visualization(&mut self, delta_time: f32, force_step: bool) {
        if matches!(self.state, AppState::Running) || force_step {
            if self.state == AppState::Running {
                self.current_delay += delta_time;

                if self.current_delay < self.target_delay {
                    return;
                }

                self.current_delay = 0.0;
            }

            if self.position == self.input.len() {
                self.state = AppState::Idle;
                return;
            }

            let delta = match self.input[self.position] {
                '^' => Translation2D::new(0, -1),
                'v' => Translation2D::new(0, 1),
                '>' => Translation2D::new(1, 0),
                '<' => Translation2D::new(-1, 0),
                _ => panic!("unexpected character"),
            };

            self.position += 1;
            self.santa_pos = delta.transform_point(self.santa_pos);

            if !self.houses.contains(&self.santa_pos) {
                self.houses.push(self.santa_pos);
            }
        }
    }

    fn render_main_vis(&self, area: Rect, buf: &mut Buffer) {
        let input_file = Line::from(self.input_path.clone());
        let title = Line::from(" AOC 2015 Day 02 ".bold());

        let instructions = Line::from(vec![
            " Continue/Pause ".into(),
            "<Enter>".blue().bold(),
            " Step ".into(),
            "<Space>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Change Speed ".into(),
            "<Up/Down>".blue().bold(),
        ]);

        let state = Line::from(self.state.to_string());
        let speed = Line::from(format!("Anim Delay: {:.0}", self.target_delay));

        let block = Block::bordered()
            .title(input_file.left_aligned())
            .title(title.centered())
            .title_bottom(instructions.centered())
            .title_bottom(state.left_aligned())
            .title_bottom(speed.right_aligned())
            .border_set(border::THICK);

        let mut map = vec![vec![' '; area.width as usize]; area.height as usize];
        for house in &self.houses {
            let pos = to_rect_coords(*house, area);
            if let Ok((x, y)) = pos {
                map[y][x] = 'A';
            }
        }
        if let Ok((x, y)) = to_rect_coords(self.santa_pos, area) {
            map[y][x] = 'S';
        }

        let map_str = map
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        let text = Text::from(map_str);

        Paragraph::new(text).block(block).render(area, buf);
    }

    fn render_houses_set(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!("Count: {}", self.houses.len()));

        let block = Block::bordered()
            .title_bottom(title.centered())
            .border_set(border::THICK);

        let houses = self
            .houses
            .iter()
            .map(|house| format!("{:3}, {:3}", house.x, house.y))
            .collect::<Vec<_>>()
            .join("\n");
        let text = Text::from(wrap_columns(&houses, block.inner(area).height.into()));

        Paragraph::new(text).block(block).render(area, buf);
    }

    fn render_bottom_input(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().border_set(border::THICK);

        let inner = block.inner(area);

        block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .split(inner);

        Text::styled(
            self.input[..self.position].iter().collect::<String>(),
            Color::DarkGray,
        )
        .right_aligned()
        .render(layout[0], buf);
        Text::styled(
            self.input[self.position].to_string(),
            (Color::Blue, Modifier::BOLD),
        )
        .centered()
        .render(layout[1], buf);
        Text::raw(self.input[self.position + 1..].iter().collect::<String>())
            .left_aligned()
            .render(layout[2], buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let delta_time = (now - self.now) as f32;
        self.now = now;
        self.tick_visualization(delta_time, false);

        let layout_vert = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .split(area);

        let houses_cols = self
            .houses
            .len()
            .div_ceil((layout_vert[0].height as usize).saturating_sub(2).max(1))
            .max(1);

        let layout_hor = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(
                    (2 + 8 * houses_cols + 2 * houses_cols.saturating_sub(1)).max(11) as u16,
                ),
            ])
            .split(layout_vert[0]);

        self.render_main_vis(layout_hor[0], buf);
        self.render_houses_set(layout_hor[1], buf);
        self.render_bottom_input(layout_vert[1], buf);
    }
}

fn to_rect_coords(point: Point2D<i32>, rect: Rect) -> Result<(usize, usize), ()> {
    let middle_x = rect.width / 2;
    let middle_y = rect.height / 2;

    let x = point.x + i32::from(middle_x);
    let y = point.y + i32::from(middle_y);

    if x >= 0 && y >= 0 {
        Ok((x as usize, y as usize))
    } else {
        Err(())
    }
}

fn wrap_columns(text: &str, height: usize) -> String {
    let lines = text.lines().collect::<Vec<_>>();

    (0..height)
        .map(|i| {
            let mut s = Vec::new();
            for j in 0..lines.len().div_ceil(height) {
                s.push(lines.get(i + j * height).map_or("", |v| v));
            }
            s.join("  ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
