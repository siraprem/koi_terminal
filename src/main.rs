use crossterm::{
    cursor,
    event::{self, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind},
    style::{Color, PrintStyledContent, Stylize},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

const INITIAL_KOI: usize = 10;
const SEGMENTS: usize = 6;
const TRAIL_LENGTH: usize = 12;

const MAX_SPEED: f32 = 0.35;
const WATER_FORCE: f32 = 0.15;

const SEPARATION_DIST: f32 = 3.0;
const ALIGNMENT_FORCE: f32 = 0.04;
const COHESION_FORCE: f32 = 0.01;
const SEPARATION_FORCE: f32 = 0.12;

const FPS: u64 = 30;

// =========================

#[derive(Clone)]
struct Koi {
    body: Vec<(f32, f32)>,
    trail: Vec<(f32, f32)>,
    vx: f32,
    vy: f32,
    base_color: Color,
    noise_offset: f32,
}

impl Koi {
    fn new(w: u16, h: u16) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(5.0..(w - 5) as f32);
        let y = rng.gen_range(3.0..(h - 3) as f32);

        let mut body = Vec::new();
        for i in 0..SEGMENTS {
            body.push((x - i as f32, y));
        }

        Self {
            body,
            trail: Vec::new(),
            vx: rng.gen_range(-MAX_SPEED..MAX_SPEED),
            vy: rng.gen_range(-MAX_SPEED..MAX_SPEED),
            base_color: match rng.gen_range(0..6) {
                0 => Color::Blue,
                1 => Color::Cyan,
                2 => Color::Green,
                3 => Color::Magenta,
                4 => Color::Yellow,
                _ => Color::Red,
            },
            noise_offset: rng.gen_range(0.0..1000.0),
        }
    }

    fn new_at_position(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();

        let mut body = Vec::new();
        for i in 0..SEGMENTS {
            body.push((x - i as f32, y));
        }

        Self {
            body,
            trail: Vec::new(),
            vx: rng.gen_range(-MAX_SPEED..MAX_SPEED),
            vy: rng.gen_range(-MAX_SPEED..MAX_SPEED),
            base_color: match rng.gen_range(0..6) {
                0 => Color::Blue,
                1 => Color::Cyan,
                2 => Color::Green,
                3 => Color::Magenta,
                4 => Color::Yellow,
                _ => Color::Red,
            },
            noise_offset: rng.gen_range(0.0..1000.0),
        }
    }

    fn apply_boids(&mut self, others: &[Koi]) {
        let (hx, hy) = self.body[0];
        let mut avg_vx = 0.0;
        let mut avg_vy = 0.0;
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        let mut count = 0;
        let mut sep_x = 0.0;
        let mut sep_y = 0.0;

        for other in others {
            let (ox, oy) = other.body[0];
            let dx = ox - hx;
            let dy = oy - hy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > 0.0 && dist < 12.0 {
                avg_vx += other.vx;
                avg_vy += other.vy;
                center_x += ox;
                center_y += oy;
                count += 1;

                if dist < SEPARATION_DIST {
                    sep_x -= dx;
                    sep_y -= dy;
                }
            }
        }

        if count > 0 {
            avg_vx /= count as f32;
            avg_vy /= count as f32;
            center_x /= count as f32;
            center_y /= count as f32;

            self.vx += (avg_vx - self.vx) * ALIGNMENT_FORCE;
            self.vy += (avg_vy - self.vy) * ALIGNMENT_FORCE;

            self.vx += (center_x - hx) * COHESION_FORCE;
            self.vy += (center_y - hy) * COHESION_FORCE;

            self.vx += sep_x * SEPARATION_FORCE;
            self.vy += sep_y * SEPARATION_FORCE;
        }
    }

    fn apply_water(&mut self) {
        let scale = 0.08;
        let t = self.noise_offset;

        let hx = self.body[0].0;
        let hy = self.body[0].1;

        let flow_x = (hy * scale + t).sin();
        let flow_y = (hx * scale + t).cos();

        self.vx += flow_x * WATER_FORCE;
        self.vy += flow_y * WATER_FORCE;

        self.noise_offset += 0.015;

        self.vx = self.vx.clamp(-MAX_SPEED, MAX_SPEED);
        self.vy = self.vy.clamp(-MAX_SPEED, MAX_SPEED);
    }

    fn update(&mut self, w: u16, h: u16) {
        let (hx, hy) = self.body[0];
        let mut nx = hx + self.vx;
        let mut ny = hy + self.vy;

        if nx <= 2.0 || nx >= (w - 2) as f32 {
            self.vx *= -1.0;
            nx = hx;
        }
        if ny <= 2.0 || ny >= (h - 2) as f32 {
            self.vy *= -1.0;
            ny = hy;
        }

        self.trail.push((hx, hy));
        if self.trail.len() > TRAIL_LENGTH {
            self.trail.remove(0);
        }

        self.body.insert(0, (nx, ny));
        if self.body.len() > SEGMENTS {
            self.body.pop();
        }
    }

    fn draw(&self, stdout: &mut std::io::Stdout) {
        for (i, &(x, y)) in self.trail.iter().enumerate() {
            let color = match self.base_color {
                Color::Red => Color::DarkRed,
                Color::Green => Color::DarkGreen,
                Color::Blue => Color::DarkBlue,
                Color::Cyan => Color::DarkCyan,
                Color::Magenta => Color::DarkMagenta,
                Color::Yellow => Color::DarkYellow,
                _ => Color::Grey,
            };

            let _ = stdout.execute(cursor::MoveTo(x as u16, y as u16));
            let _ = stdout.execute(PrintStyledContent("·".with(color)));
        }

        for (i, &(x, y)) in self.body.iter().enumerate() {
            let color = if i == 0 {
                self.base_color
            } else {
                match self.base_color {
                    Color::Red => Color::DarkRed,
                    Color::Green => Color::DarkGreen,
                    Color::Blue => Color::DarkBlue,
                    Color::Cyan => Color::DarkCyan,
                    Color::Magenta => Color::DarkMagenta,
                    Color::Yellow => Color::DarkYellow,
                    _ => Color::Grey,
                }
            };

            let _ = stdout.execute(cursor::MoveTo(x as u16, y as u16));
            let _ = stdout.execute(PrintStyledContent("●".with(color)));
        }
    }

    fn distance_to(&self, x: f32, y: f32) -> f32 {
        let (kx, ky) = self.body[0];
        ((kx - x).powi(2) + (ky - y).powi(2)).sqrt()
    }
}

fn cleanup_terminal() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    terminal::disable_raw_mode()?;
    stdout.execute(event::DisableMouseCapture)?;
    stdout.execute(cursor::Show)?;
    stdout.execute(Clear(ClearType::All))?;
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar handler para Ctrl+C
    ctrlc::set_handler(move || {
        let _ = cleanup_terminal();
        std::process::exit(0);
    })?;

    let mut stdout = stdout();
    
    terminal::enable_raw_mode()?;
    stdout.execute(event::EnableMouseCapture)?;
    stdout.execute(cursor::Hide)?;

    let (mut w, mut h) = terminal::size()?;
    let mut kois: Vec<Koi> = (0..INITIAL_KOI).map(|_| Koi::new(w, h)).collect();

    let frame_duration = Duration::from_millis(1000 / FPS);
    let mut last_frame = std::time::Instant::now();
    let mut running = true;

    while running {
        // Processar eventos sem bloquear
        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                            running = false;
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event {
                        MouseEvent {
                            kind: MouseEventKind::Down(MouseButton::Left),
                            column,
                            row,
                            ..
                        } => {
                            kois.push(Koi::new_at_position(column as f32, row as f32));
                        }
                        MouseEvent {
                            kind: MouseEventKind::Down(MouseButton::Right),
                            column,
                            row,
                            ..
                        } => {
                            if !kois.is_empty() {
                                let click_x = column as f32;
                                let click_y = row as f32;
                                
                                let mut closest_idx = 0;
                                let mut min_dist = kois[0].distance_to(click_x, click_y);
                                
                                for (i, koi) in kois.iter().enumerate().skip(1) {
                                    let dist = koi.distance_to(click_x, click_y);
                                    if dist < min_dist {
                                        min_dist = dist;
                                        closest_idx = i;
                                    }
                                }
                                
                                if min_dist < 5.0 {
                                    kois.remove(closest_idx);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_frame.elapsed() >= frame_duration {
            stdout.execute(Clear(ClearType::All))?;
            (w, h) = terminal::size()?;

            let snapshot = kois.clone();

            for koi in kois.iter_mut() {
                koi.apply_boids(&snapshot);
                koi.apply_water();
                koi.update(w, h);
                koi.draw(&mut stdout);
            }

            stdout.flush()?;
            last_frame = std::time::Instant::now();
        }
    }

    cleanup_terminal()?;
    Ok(())
}