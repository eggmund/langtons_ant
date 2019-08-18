use ggez::nalgebra as na;
use na::Point2;

use crate::GRID_SIZE;

pub struct Ant {
    pub pos: Point2<usize>,
    pub direction: Direction,
}

impl Default for Ant {
    fn default() -> Ant {
        Ant {
            pos: Point2::new(GRID_SIZE/2, GRID_SIZE/2),
            direction: Direction::Up,
        }
    }
}

impl Ant {
    pub fn step(&mut self, grid: &mut [[bool; GRID_SIZE]; GRID_SIZE]) {
        if grid[self.pos.x][self.pos.y] {   // White
            self.direction.turn_right();
        } else {
            self.direction.turn_left();
        }

        grid[self.pos.x][self.pos.y] = !grid[self.pos.x][self.pos.y];
        self.move_forward();
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::Left => {
                if self.pos.x > 0 {
                    self.pos.x -= 1;
                }
            },
            Direction::Right => {
                if self.pos.x < GRID_SIZE-1 {
                    self.pos.x += 1;
                }
            },
            Direction::Up => {
                if self.pos.y > 0 {
                    self.pos.y -= 1;
                }
            },
            Direction::Down => {
                if self.pos.y < GRID_SIZE-1 {
                    self.pos.y += 1;
                }
            },
        };
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match *self {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }

    fn turn_right(&mut self) {
        *self = match *self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    pub fn in_radians(&self) -> f32 {
        use std::f32::consts;
        match *self {
            Direction::Left => consts::FRAC_PI_2 * 3.0, // 270.0,
            Direction::Up => 0.0,
            Direction::Right => consts::FRAC_PI_2,
            Direction::Down => consts::PI,
        }
    }

    pub fn in_degrees(&self) -> f32 {
        match *self {
            Direction::Left => 270.0,
            Direction::Up => 0.0,
            Direction::Right => 90.0,
            Direction::Down => 180.0,
        }
    }
}