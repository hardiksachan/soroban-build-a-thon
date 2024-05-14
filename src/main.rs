use std::{fmt::Display, thread};

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right
        } else if frame.width <= self.x {
            self.horiz_dir = HorizDir::Left
        }

        if self.y <= 0 {
            self.vert_dir = VertDir::Up
        } else if frame.height <= self.y {
            self.vert_dir = VertDir::Down
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Down => self.y -= 1,
            VertDir::Up => self.y += 1,
        }
    }
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    ball: Ball,
    frame: Frame,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                width: 63,
                height: 11,
            },
            ball: Ball {
                x: 44,
                y: 1,
                horiz_dir: HorizDir::Left,
                vert_dir: VertDir::Down,
            },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..25 {
            write!(f, "\n")?;
        }
        for y in 0..=self.frame.height {
            for x in 0..=self.frame.width {
                if self.ball.x == x && self.ball.y == y {
                    write!(f, "O")?;
                } else if x == 0 || x == self.frame.width {
                    write!(f, "|")?;
                } else if y == 0 || y == self.frame.height {
                    write!(f, "-")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

fn main() {
    let mut game = Game::new();
    let sleep_time = std::time::Duration::from_millis(30);
    loop {
        println!("{game}");
        game.step();
        thread::sleep(sleep_time);
    }
}
