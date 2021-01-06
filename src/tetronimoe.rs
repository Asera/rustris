use rand::random;
use crate::game_state::FIELD_WIDTH;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None // Added to mark void or impossible direction state
}

impl Direction {
    pub fn next_ccw(current: Direction) -> Direction {
        return match current {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            _ => unreachable!(),
        }
    }

    pub fn next_cw(current: Direction) -> Direction {
        return match current {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum BlockType {
    Square,
    Squiggle,
    ReverseSquiggle,
    LBlock,
    ReverseLBlock,
    TBlock,
    LinePiece
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone)]
pub struct Figure {
    pub block_type: BlockType,
    pub direction: Direction,
    pub center: Point, // center is basically center of rotation for the figure.
    // It is asymmetrical, but full circle CW/CCW rotation will held figure intact
    // Center position depends on figure type:
    //   * Square - left lower block, just because
    //   * Squiggle - lower central block(i.e. right square in lower row)
    //   * ReverseSquiggle - lower central block (i.e. left square in lower row)
    //   * LBlock - block connecting | and _
    //   * ReverseLBlock - block connecting | and _
    //   * TBlock - block connecting | and _
    //   * LinePiece - second block from below
    pub tiles: Vec<Point>,
}

impl Figure {
    pub fn init(block_type: BlockType, direction: Direction, center: Point) -> Figure {
        let tiles = calculate_tiles_position(&block_type, &direction, center);
        Figure { block_type, direction, center, tiles }
    }

    pub fn init_random() -> Figure {
        let block_type = match random::<u32>() % 7 {
            0 => BlockType::Square,
            1 => BlockType::Squiggle,
            2 => BlockType::ReverseSquiggle,
            3 => BlockType::LBlock,
            4 => BlockType::ReverseLBlock,
            5 => BlockType::TBlock,
            6 => BlockType::LinePiece,
            _ => unimplemented!(),
        };

        let center = Point {x: (FIELD_WIDTH / 2) as isize, y: 2};
        let tiles = calculate_tiles_position(&block_type, &Direction::Up, center);

        Figure {
            block_type,
            direction: Direction::Up,
            center,
            tiles
        }
    }

    // Note that rotation does not work according to SRS.
    // Just because I decided to make experiment with it.
    pub fn rotate_cw(&mut self) {
        self.direction = Direction::next_cw(self.direction.clone());
        self.tiles = calculate_tiles_position(&self.block_type, &self.direction, self.center);
    }

    // Note that rotation does not work according to SRS.
    // Just because I decided to make experiment with it.
    pub fn rotate_ccw(&mut self) {
        self.direction = Direction::next_ccw(self.direction.clone());
        self.tiles = calculate_tiles_position(&self.block_type, &self.direction, self.center);
    }

    pub fn get_tiles(&self) -> Vec<Point> {
        self.tiles.clone()
    }

    pub fn shift_right(&mut self) {
        self.center.x += 1;
        self.tiles = calculate_tiles_position(&self.block_type, &self.direction, self.center);
    }

    pub fn shift_left(&mut self) {
        self.center.x -= 1;
        self.tiles = calculate_tiles_position(&self.block_type, &self.direction, self.center);
    }

    pub fn shift_down(&mut self) {
        self.center.y += 1;
        self.tiles = calculate_tiles_position(&self.block_type, &self.direction, self.center);
    }

    pub fn get_start_position() -> Point {
        Point {x: (FIELD_WIDTH / 2) as isize, y: 2}
    }
}

fn calculate_tiles_position(block_type: &BlockType, direction: &Direction, center: Point) -> Vec<Point> {
    return match block_type {
        BlockType::Square => {
            vec![
                center.clone(), // lower left
                Point {
                    x: center.x,
                    y: center.y - 1
                }, // higher left
                Point {
                    x: center.x + 1,
                    y: center.y - 1
                }, // higher right
                Point {
                    x: center.x + 1,
                    y: center.y
                }, // lower right
            ]
        },
        BlockType::Squiggle => get_squiggle_tiles(&direction, center),
        BlockType::ReverseSquiggle => get_reverse_squiggle_tiles(&direction, center),
        BlockType::LBlock => get_l_block_tiles(&direction, center),
        BlockType::ReverseLBlock => get_reverse_l_block_tiles(&direction, center),
        BlockType::TBlock => get_t_block_tiles(&direction, center),
        BlockType::LinePiece => get_line_piece_tiles(&direction, center),
    };
}

fn get_squiggle_tiles(direction: &Direction, center: Point) -> Vec<Point> {
    return match direction {
        Direction::Up => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y + 1
            },// lower left
            Point {
                x: center.x - 1,
                y: center.y - 1
            },// upper right
            Point {
                x: center.x - 1,
                y: center.y
            },// upper left
        ],
        Direction::Right => vec![
            center.clone(),
            Point {
                x: center.x - 1,
                y: center.y
            },// lower left
            Point {
                x: center.x + 1,
                y: center.y - 1
            },// upper right
            Point {
                x: center.x,
                y: center.y - 1
            },// upper left
        ],
        Direction::Down => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y - 1
            },// lower left
            Point {
                x: center.x + 1,
                y: center.y + 1
            },// upper right
            Point {
                x: center.x + 1,
                y: center.y
            },// upper left
        ],
        Direction::Left => vec![
            center.clone(),
            Point {
                x: center.x + 1,
                y: center.y
            },// lower left
            Point {
                x: center.x - 1,
                y: center.y + 1
            },// upper right
            Point {
                x: center.x,
                y: center.y + 1
            },// upper left
        ],
        _ => unreachable!(),
    }
}

fn get_reverse_squiggle_tiles(direction: &Direction, center: Point) -> Vec<Point> {
    return match direction {
        Direction::Up => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y + 1
            },// lower right
            Point {
                x: center.x + 1,
                y: center.y
            },// upper right
            Point {
                x: center.x + 1,
                y: center.y - 1
            },// upper left
        ],
        Direction::Right => vec![
            center.clone(),
            Point {
                x: center.x + 1,
                y: center.y
            },// lower right
            Point {
                x: center.x,
                y: center.y - 1
            },// upper right
            Point {
                x: center.x - 1,
                y: center.y - 1
            },// upper left
        ],
        Direction::Down => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y - 1
            },// lower right
            Point {
                x: center.x - 1,
                y: center.y
            },// upper right
            Point {
                x: center.x - 1,
                y: center.y + 1
            },// upper left
        ],
        Direction::Left => vec![
            center.clone(),
            Point {
                x: center.x - 1,
                y: center.y
            },// lower right
            Point {
                x: center.x,
                y: center.y + 1
            },// upper right
            Point {
                x: center.x + 1,
                y: center.y + 1
            },// upper left
        ],
        _ => unreachable!(),
    }
}

fn get_l_block_tiles(direction: &Direction, center: Point) -> Vec<Point> {
    return match direction {
        Direction::Up => vec![
            center.clone(),
            Point {
                x: center.x + 1,
                y: center.y
            },// lower right
            Point {
                x: center.x,
                y: center.y - 1
            },// block right above
            Point {
                x: center.x,
                y: center.y - 2
            },// 1 block above
        ],
        Direction::Right => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y + 1
            },// lower right
            Point {
                x: center.x + 1,
                y: center.y
            },// block right above
            Point {
                x: center.x + 2,
                y: center.y
            },// 1 block above
        ],
        Direction::Down => vec![
            center.clone(),
            Point {
                x: center.x - 1,
                y: center.y
            },// lower right
            Point {
                x: center.x,
                y: center.y + 1
            },// block right above
            Point {
                x: center.x,
                y: center.y + 2
            },// 1 block above
        ],
        Direction::Left => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y - 1
            },// lower right
            Point {
                x: center.x - 1,
                y: center.y
            },// block right above
            Point {
                x: center.x - 2,
                y: center.y
            },// 1 block above
        ],
        _ => unreachable!(),
    }
}

fn get_reverse_l_block_tiles(direction: &Direction, center: Point) -> Vec<Point> {
    return match direction {
        Direction::Up => vec![
            center.clone(),
            Point {
                x: center.x - 1,
                y: center.y
            },// lower left
            Point {
                x: center.x,
                y: center.y - 1
            },// block right above
            Point {
                x: center.x,
                y: center.y - 2
            },// 1 block above
        ],
        Direction::Right => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y - 1
            },// lower left
            Point {
                x: center.x + 1,
                y: center.y
            },// block right above
            Point {
                x: center.x + 2,
                y: center.y
            },// 1 block above
        ],
        Direction::Down => vec![
            center.clone(),
            Point {
                x: center.x + 1,
                y: center.y
            },// lower left
            Point {
                x: center.x,
                y: center.y + 1
            },// block right above
            Point {
                x: center.x,
                y: center.y + 2
            },// 1 block above
        ],
        Direction::Left => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y + 1
            },// lower left
            Point {
                x: center.x - 1,
                y: center.y
            },// block right above
            Point {
                x: center.x - 2,
                y: center.y
            },// 1 block above
        ],
        _ => unreachable!(),
    }
}

fn get_t_block_tiles(direction: &Direction, center: Point) -> Vec<Point> {
    return match direction {
        Direction::Up => vec![
            center.clone(),
            Point {
                x: center.x - 1,
                y: center.y
            },// lower left
            Point {
                x: center.x + 1,
                y: center.y
            },// lower right
            Point {
                x: center.x,
                y: center.y - 1
            },// upper
        ],
        Direction::Right => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y - 1
            },// lower left
            Point {
                x: center.x,
                y: center.y + 1
            },// lower right
            Point {
                x: center.x + 1,
                y: center.y
            },// upper
        ],
        Direction::Down => vec![
            center.clone(),
            Point {
                x: center.x + 1,
                y: center.y
            },// lower left
            Point {
                x: center.x - 1,
                y: center.y
            },// lower right
            Point {
                x: center.x,
                y: center.y + 1
            },// upper
        ],
        Direction::Left => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y + 1
            },// lower left
            Point {
                x: center.x,
                y: center.y - 1
            },// lower right
            Point {
                x: center.x - 1,
                y: center.y
            },// upper
        ],
        _ => unreachable!(),
    }
}

fn get_line_piece_tiles(direction: &Direction, center: Point) -> Vec<Point> {
    return match direction {
        Direction::Up => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y - 1
            },// block right above center
            Point {
                x: center.x,
                y: center.y + 1
            },// 1 block above center
            Point {
                x: center.x,
                y: center.y + 2
            },// block right below center
        ],
        Direction::Right => vec![
            center.clone(),
            Point {
                x: center.x + 1,
                y: center.y
            },// block right above center
            Point {
                x: center.x - 1,
                y: center.y
            },// 1 block above center
            Point {
                x: center.x - 2,
                y: center.y
            },// block right below center
        ],
        Direction::Down => vec![
            center.clone(),
            Point {
                x: center.x,
                y: center.y + 1
            },// block right above center
            Point {
                x: center.x,
                y: center.y - 1
            },// 1 block above center
            Point {
                x: center.x,
                y: center.y - 2
            },// block right below center
        ],
        Direction::Left => vec![
            center.clone(),
            Point {
                x: center.x - 1,
                y: center.y
            },// block right above center
            Point {
                x: center.x + 1,
                y: center.y
            },// 1 block above center
            Point {
                x: center.x + 2,
                y: center.y
            },// block right below center
        ],
        _ => unreachable!(),
    }
}
