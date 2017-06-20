use std::collections::VecDeque;
use std::iter;

use rand::{thread_rng, sample};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    NoDirection,
}


#[derive(Debug, Default, Eq, PartialEq)]
struct Square {
    x: u32,
    y: u32,
    // food: u32,
}


impl Square {
    fn new(x: u32, y: u32) -> Square {
        Square {
            x: x,
            y: y,
            // food: 0,
        }
    }
    pub fn x(&self) -> u32 {
        self.x
    }
    pub fn y(&self) -> u32 {
        self.y
    }
}


pub struct Food {
    square: Square,
    points: u32,
    segments: u32,
}


impl Food {
    pub fn generate(on_grid: &Grid, with_snake: &Snake) -> Food {
        let mut rng = thread_rng();
        let free_squares = on_grid.squares.iter()
            .filter(|s| !with_snake.collides(s));
        let sample = sample(&mut rng, free_squares, 1);
        let square = sample.get(0).unwrap();
        Food {
            square: Square { x: square.x, y: square.y },
            points: 5u32,
            segments: 3u32,
        }
    }

    pub fn points(&self) -> u32 {
        self.points
    }

    pub fn get_square(&self) -> &Square {
        &self.square
    }
}


pub struct Snake {
    direction: Direction,
    squares: VecDeque<Square>,
    pending: u32,
}


impl Snake {
    pub fn new(start_x: u32, start_y: u32) -> Snake {
        let mut squares: VecDeque<Square> = VecDeque::new();
        squares.push_front(Square { x: start_x, y: start_y });
        Snake {
            direction: Direction::NoDirection,
            squares: squares,
            pending: 5u32,
        }
    }

    pub fn traverse(&mut self, on_grid: &Grid) -> bool {
        let next = on_grid.next_square(self.squares.back().unwrap(), self.direction);
        if self.collides(&next) {
            return false
        }
        self.squares.push_back(next);
        if self.pending == 0 {
            self.squares.pop_front();
        } else {
            self.pending -= 1;
        }
        true
    }

    pub fn squares(&self) -> &VecDeque<Square> {
        &self.squares
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn set_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    fn collides(&self, other: &Square) -> bool {
        self.squares.contains(other)
    }
}


pub struct Grid {
    width: u32,
    height: u32,
    squares: Vec<Square>,
}


impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let squares = (0..height)
            .map(|y| iter::repeat(y).zip(0..width))
            .flat_map(|yxs| yxs)
            .map(|yx| Square { x: yx.1, y: yx.0 })
            .collect();
        Grid {
            width: width,
            height: height,
            squares: squares,
        }
    }

    fn next_square(&self, location: &Square, dir: Direction) -> Square {
        let mut x = location.x;
        let mut y = location.y;
        match dir {
            Direction::Up => {
                if y > 0 {
                    y -= 1;
                } else {
                    y = self.height - 1;
                }
            },
            Direction::Right => {
                if x < self.width - 1 {
                    x += 1;
                } else {
                    x = 0;
                }
            },
            Direction::Down => {
                if y < self.height - 1 {
                    y += 1;
                } else {
                    y = 0;
                }
            },
            Direction::Left => {
                if x > 0 {
                    x -= 1;
                } else {
                    x = self.width - 1;
                }
            },
            Direction::NoDirection => {}
        }
        Square { x, y }
    }
}


mod square_tests {
    use super::*;

    #[test]
    fn square_equality() {
        let s1 = Square { x: 10, y: 20 };
        let s2 = Square { x: 10, y: 20 };
        assert_eq!(s1, s2);
    }
}


mod grid_tests {
    use super::*;

    #[test]
    fn grid_next_incr() {
        let grid = Grid::new(12, 10);
        let expected = vec![
            Square { x: 5, y: 7 },
            Square { x: 5, y: 7 },
            Square { x: 5, y: 7 },
            Square { x: 5, y: 7 },
            Square { x: 5, y: 7 },
        ];
        let actual = vec![
            grid.next_square(&Square { x: 5, y: 8 }, Direction::Up),
            grid.next_square(&Square { x: 4, y: 7 }, Direction::Right),
            grid.next_square(&Square { x: 5, y: 6 }, Direction::Down),
            grid.next_square(&Square { x: 6, y: 7 }, Direction::Left),
            grid.next_square(&Square { x: 5, y: 7 }, Direction::NoDirection),
        ];
        let zipped = expected.iter().zip(actual.iter());
        for (expected, actual) in zipped {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn grid_next_overwrapped() {
        let grid = Grid::new(12, 10);
        let expected = vec![
            Square { x: 0, y: 5 },
            Square { x: 5, y: 0 },
            Square { x: 11, y: 5 },
            Square { x: 5, y: 9 },
        ];
        let actual = vec![
            grid.next_square(&Square { x: 11, y: 5 }, Direction::Right),
            grid.next_square(&Square { x: 5, y: 9 }, Direction::Down),
            grid.next_square(&Square { x: 0, y: 5 }, Direction::Left),
            grid.next_square(&Square { x: 5, y: 0 }, Direction::Up),
        ];
        let zipped = expected.iter().zip(actual.iter());
        for (expected, actual) in zipped {
            assert_eq!(expected, actual);
        }
    }
}
