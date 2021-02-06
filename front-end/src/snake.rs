pub const WIDTH: u32 = 300;
pub const HEIGHT: u32 = 300;
pub const LINE_THICKNESS: f64 = 25.0;

#[derive(Clone, Debug)]
pub struct Snake {
    pub body: Vec<Position>,
    pub direction: Direction,
    pub target: Option<Position>,
    pub alive: bool,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![Position { x: 0.0, y: 0.0 }],
            direction: Direction::Right,
            target: Some(Position::random()),
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

    pub fn apple_count(&self) -> usize {
        self.body.len() - 1
    }

    pub fn move_along(&self) -> (Snake, Option<Position>) {
        if !self.alive {
            return (self.clone(), None);
        }
        if self.dying() {
            return (self.kill(), None);
        }

        let new_head = self.next_position();
        let (dropped, target, body) = if Some(new_head) == self.target {
            let mut body = self.body.clone();
            body.push(new_head);
            (None, Position::random_except(&body), body)
        } else {
            let mut body = self.body.iter().skip(1).copied().collect::<Vec<_>>();
            body.push(new_head);
            (Some(*self.tail()), self.target, body)
        };

        (
            Snake {
                body,
                target,
                ..*self
            },
            dropped,
        )
    }

    fn dying(&self) -> bool {
        let next_pos = self.next_position();
        !next_pos.is_inside_walls()
            || self
                .body
                .iter()
                .skip(if Some(next_pos) == self.target { 0 } else { 1 })
                .any(|p| *p == next_pos)
    }

    pub fn kill(&self) -> Snake {
        Snake {
            alive: false,
            ..self.clone()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn turn_180_degrees(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    fn random() -> Position {
        let mut x = (rand::random::<f64>() * WIDTH as f64).floor();
        let mut y = (rand::random::<f64>() * HEIGHT as f64).floor();
        // we substract val % LINE_THICKNESS so the snake can get here
        x -= x % LINE_THICKNESS;
        y -= y % LINE_THICKNESS;
        Position { x, y }
    }

    fn random_except(blacklist: &[Position]) -> Option<Position> {
        let max_positions = WIDTH / LINE_THICKNESS as u32 * HEIGHT / LINE_THICKNESS as u32;
        // TODO: Maybe don't do completely random when there are only a few options
        if blacklist.len() as u32 == max_positions {
            return None;
        }
        loop {
            let random = Position::random();
            if blacklist.iter().all(|p| *p != random) {
                return Some(random);
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
    fn its_next_position_is_thickness_away_from_head() {
        let snake = Snake::new();
        let head = snake.head();

        let next = snake.next_position();

        assert_eq!(next.x, head.x + LINE_THICKNESS);
    }

    #[test]
    fn it_dies_when_crashing_into_wall() {
        let snake = Snake {
            direction: Direction::Left,
            body: vec![Position { x: 0.0, y: 0.0 }],
            ..Snake::new()
        };

        assert!(snake.dying());
        let (snake, _) = snake.move_along();
        assert_eq!(false, snake.alive);
    }

    #[test]
    fn it_dies_when_crashing_into_self() {
        let snake = Snake {
            body: vec![
                Position { x: 0.0, y: 0.0 },
                Position { x: 25.0, y: 0.0 },
                Position { x: 50.0, y: 0.0 },
                Position { x: 50.0, y: 25.0 },
                Position { x: 25.0, y: 25.0 },
            ],
            direction: Direction::Up,
            ..Snake::new()
        };

        assert!(snake.dying());
        let (snake, _) = snake.move_along();
        assert_eq!(false, snake.alive)
    }

    #[test]
    fn it_lives_when_moving_its_head_to_where_its_tail_was() {
        let snake = Snake {
            body: vec![
                Position { x: 0.0, y: 0.0 },
                Position { x: 25.0, y: 0.0 },
                Position { x: 25.0, y: 25.0 },
                Position { x: 0.0, y: 25.0 },
            ],
            target: Position::random_except(&vec![Position { x: 0.0, y: 0.0 }]),
            direction: Direction::Up,
            ..Snake::new()
        };

        assert_eq!(false, snake.dying());
        let (snake, _) = snake.move_along();
        assert!(snake.alive)
    }

    #[test]
    fn it_dies_when_killed() {
        let snake = Snake::new();
        assert_eq!(false, snake.kill().alive);
    }

    #[test]
    fn it_moves_its_tail_when_moving() {
        let snake = Snake {
            target: Some(Position { x: 100.0, y: 100.0 }),
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
            target: Some(Position {
                x: LINE_THICKNESS,
                y: 0.0,
            }),
            ..Snake::new()
        };
        let original_tail = *snake.tail();

        let (snake, _) = snake.move_along();

        assert_eq!(original_tail, *snake.tail());
        assert_eq!(2, snake.body.len());
    }

    #[test]
    fn position_is_inside_walls_should_be_true() {
        assert!(Position { x: 0.0, y: 0.0 }.is_inside_walls());
        assert!(Position { x: 50.0, y: 50.0 }.is_inside_walls());
        assert!(Position {
            x: (WIDTH - 1) as f64,
            y: (HEIGHT - 1) as f64
        }
        .is_inside_walls());
    }

    #[test]
    fn position_is_inside_walls_should_be_false() {
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

    #[test]
    fn apple_count_should_initially_be_zero() {
        assert_eq!(0, Snake::new().apple_count());
    }

    #[test]
    fn apple_count_should_increase_when_eating_apple() {
        let snake = Snake {
            direction: Direction::Right,
            target: Some(Position {
                x: LINE_THICKNESS,
                y: 0.0,
            }),
            ..Snake::new()
        };
        let (snake, _) = snake.move_along();

        assert_eq!(1, snake.apple_count());
    }

    #[test]
    fn it_should_be_possible_to_fill_the_whole_screen_with_snake() {
        let mut snake = Snake::new();
        let bodies_per_line = WIDTH / LINE_THICKNESS as u32;
        let max_length = bodies_per_line.pow(2);
        for _ in 0..max_length {
            snake.direction = match snake.direction {
                Direction::Down if snake.head().x == 0.0 => Direction::Right,
                Direction::Down => Direction::Left,
                _ if snake.next_position().is_inside_walls() => snake.direction,
                _ => Direction::Down,
            };
            snake.target = Some(snake.next_position());
            snake = snake.move_along().0;
        }
        assert_eq!(max_length, snake.body.len() as u32);
    }

    #[test]
    fn direction_turn_180_degrees_given_up_should_be_down() {
        assert_eq!(Direction::Up, Direction::Down.turn_180_degrees());
    }
}
