use rand;

pub const WIDTH: u32 = 300;
pub const HEIGHT: u32 = 300;
pub const LINE_THICKNESS: f64 = 25.0;

#[derive(Debug)]
pub struct Snake {
    pub body: Vec<Position>,
    pub direction: Direction,
    pub target: Position,
    pub alive: bool,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![Position { x: 0.0, y: 0.0 }],
            direction: Direction::Right,
            target: Position::random(),
            alive: true,
        }
    }

    fn next_position(&self) -> Position {
        let head = self.head();
        match self.direction {
            Direction::Up => Position {
                y: head.y - LINE_THICKNESS,
                ..*head
            },
            Direction::Right => Position {
                x: head.x + LINE_THICKNESS,
                ..*head
            },
            Direction::Down => Position {
                y: head.y + LINE_THICKNESS,
                ..*head
            },
            Direction::Left => Position {
                x: head.x - LINE_THICKNESS,
                ..*head
            },
        }
    }

    pub fn head(&self) -> &Position {
        self.body
            .last()
            .expect("snake has no head because it has no body")
    }

    pub fn tail(&self) -> &Position {
        self.body
            .first()
            .expect("snake has no tail because it has no body")
    }

    pub fn move_along(&self) -> (Snake, Option<Position>) {
        let new_head = self.next_position();
        let alive =
            self.alive && new_head.is_inside_walls() && !self.body.iter().any(|p| *p == new_head);

        if !self.alive {
            return (
                Snake {
                    body: self.body.clone(),
                    alive,
                    ..*self
                },
                None,
            );
        }

        let (dropped, target, mut body) = if new_head == self.target {
            (None, Position::random_except(&self.body), self.body.clone())
        } else {
            (
                Some(*self.tail()),
                self.target,
                self.body.iter().skip(1).copied().collect(),
            )
        };

        body.push(new_head);

        (
            Snake {
                body,
                target,
                alive,
                ..*self
            },
            dropped,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    fn random() -> Position {
        let mut x = (rand::random::<f64>() * WIDTH as f64).round();
        let mut y = (rand::random::<f64>() * HEIGHT as f64).round();
        // we substract val % LINE_THICKNESS so the snake can get here
        x -= x % LINE_THICKNESS;
        y -= y % LINE_THICKNESS;
        Position { x, y }
    }

    fn random_except(blacklist: &Vec<Position>) -> Position {
        loop {
            let random = Position::random();
            if blacklist.iter().all(|p| *p != random) {
                return random;
            }
        }
    }

    fn is_inside_walls(&self) -> bool {
        self.x.round() as i32 >= 0
            && (self.x.round() as u32) < WIDTH
            && self.y.round() as i32 >= 0
            && (self.y.round() as u32) < HEIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initially_moves_right() {
        let snake = Snake::new();
        assert_eq!(Direction::Right, snake.direction);
    }

    #[test]
    fn it_initially_lives() {
        let snake = Snake::new();
        assert!(snake.alive);
    }

    #[test]
    fn next_position_is_thickness_away_from_head() {
        let snake = Snake::new();
        let head = snake.head();

        let next = snake.next_position();

        assert_eq!(next.x, head.x + LINE_THICKNESS);
    }

    #[test]
    fn it_dies_when_crashing_into_wall() {
        let snake = Snake {
            direction: Direction::Left,
            ..Snake::new()
        };

        let (snake, _) = snake.move_along();

        assert_eq!(false, snake.alive);
    }

    #[test]
    fn it_moves_its_tail_when_moving() {
        let snake = Snake {
            target: Position { x: 100.0, y: 100.0 },
            ..Snake::new()
        };
        let original_tail = *snake.tail();

        let (snake, _) = snake.move_along();

        assert_ne!(original_tail, *snake.tail());
        assert_eq!(1, snake.body.len());
    }

    #[test]
    fn it_keeps_its_tail_and_gets_longer_when_eating_apple() {
        let snake = Snake {
            direction: Direction::Right,
            target: Position {
                x: LINE_THICKNESS,
                y: 0.0,
            },
            ..Snake::new()
        };
        let original_tail = *snake.tail();

        let (snake, _) = snake.move_along();

        assert_eq!(original_tail, *snake.tail());
        assert_eq!(2, snake.body.len());
    }

    #[test]
    fn is_inside_walls_should_be_true() {
        assert!(Position { x: 0.0, y: 0.0 }.is_inside_walls());
        assert!(Position { x: 50.0, y: 50.0 }.is_inside_walls());
        assert!(Position {
            x: (WIDTH - 1) as f64,
            y: (HEIGHT - 1) as f64
        }
        .is_inside_walls());
    }

    #[test]
    fn is_inside_walls_should_be_false() {
        assert_eq!(false, Position { x: -25.0, y: 0.0 }.is_inside_walls());
        assert_eq!(
            false,
            Position {
                x: WIDTH as f64,
                y: 0.0
            }
            .is_inside_walls()
        );
        assert_eq!(false, Position { x: 0.0, y: -25.0 }.is_inside_walls());
        assert_eq!(
            false,
            Position {
                x: 0.0,
                y: HEIGHT as f64,
            }
            .is_inside_walls()
        );
    }
}
