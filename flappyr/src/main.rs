extern crate rand;

use rand::Rng;
use crossterm::{cursor, event, execute, terminal};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

const GRAVITY: f32 = 0.6;
const FLAP_FORCE: f32 = -5.0;
const TERMINAL_WIDTH: u16 = 80;
const TERMINAL_HEIGHT: u16 = 25;
const OBSTACLE_WIDTH: u16 = 10;
const OBSTACLE_GAP: u16 = 8;
const GAME_SPEED: u64 = 100;

struct Bird {
    x: u16,
    y: u16,
    velocity: f32,
}

struct Obstacle {
    x: u16,
    gap_y: u16,
}

impl Bird {
    fn new() -> Bird {
        Bird {
            x: 10,
            y: TERMINAL_HEIGHT / 2,
            velocity: 0.0,
        }
    }

    fn flap(&mut self) {
        self.velocity = FLAP_FORCE;
    }

    fn update(&mut self) {
        self.y = (self.y as f32 + self.velocity) as u16;
        self.velocity += GRAVITY;
    }
}

impl Obstacle {
    fn new(x: u16, gap_y: u16) -> Obstacle {
        Obstacle { x, gap_y }
    }

    fn update(&mut self) {
        self.x -= 1;
    }

    fn draw(&self) {
        for y in 0..self.gap_y {
            print!("{}{}", cursor::MoveTo(self.x, y), "#");
        }
        for y in self.gap_y + OBSTACLE_GAP..TERMINAL_HEIGHT {
            print!("{}{}", cursor::MoveTo(self.x, y), "#");
        }
    }
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    let mut bird = Bird::new();
    let mut obstacles = vec![Obstacle::new(TERMINAL_WIDTH, TERMINAL_HEIGHT / 2)];

    let mut last_flap = Instant::now();

    loop {
        // Handle input
        if event::poll(Duration::default())? {
            if let event::Event::Key(key_event) = event::read()? {
                if key_event.code == event::KeyCode::Char(' ') && last_flap.elapsed().as_millis() > 100 {
                    bird.flap();
                    last_flap = Instant::now();
                }
            }
        }

        // Update game state
        bird.update();
        for obstacle in obstacles.iter_mut() {
            obstacle.update();
        }
        obstacles.retain(|o| o.x > 0);
        if let Some(last) = obstacles.last() {
            if last.x < TERMINAL_WIDTH - OBSTACLE_WIDTH - OBSTACLE_GAP {
                obstacles.push(Obstacle::new(TERMINAL_WIDTH, rand::random::<u16>() % (TERMINAL_HEIGHT - OBSTACLE_GAP)));
            }
        }

        // Draw game
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        print!("{}{}", cursor::MoveTo(bird.x, bird.y), "@");
        for obstacle in obstacles.iter() {
            obstacle.draw();
        }

        // Check for collisions
        if bird.y >= TERMINAL_HEIGHT || obstacles.iter().any(|o| o.x <= bird.x && bird.x < o.x + OBSTACLE_WIDTH && (bird.y < o.gap_y || bird.y >= o.gap_y + OBSTACLE_GAP)) {
            break;
        }

        // Display frame
        stdout.flush()?;
        thread::sleep(Duration::from_millis(GAME_SPEED));
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
