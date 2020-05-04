use js_sys::Math;

pub const WIDTH: f64 = 300.0;
pub const HEIGHT: f64 = 300.0;
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

    fn head(&self) -> &Position {
        self.body
            .last()
            .expect("snake has no head because it has no body")
    }

    fn tail(&self) -> &Position {
        self.body
            .last()
            .expect("snake has no tail because it has no body")
    }

    pub fn move_along(&self) -> (Snake, Option<Position>) {
        let new_head = self.next_position();

        let alive = new_head.is_inside_walls();

        let (dropped, target, mut body) = if new_head == self.target {
            (None, Position::random(), self.body.clone())
        } else {
            (
                Some(*self.tail()),
                self.target,
                self.body.iter().skip(1).copied().collect(),
            )
        };

        body.push(self.next_position());

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
        Position {
            x: Math::random() * WIDTH,
            y: Math::random() * HEIGHT,
        }
    }

    fn is_inside_walls(&self) -> bool {
        self.x >= 0.0 && self.x <= WIDTH && self.y >= 0.0 && self.y <= HEIGHT
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
    fn next_position_is_thickness_away_from_head() {
        let snake = Snake::new();
        let head = snake.head();
        let next = snake.next_position();
        assert_eq!(next.x, head.x + LINE_THICKNESS);
    }
}
