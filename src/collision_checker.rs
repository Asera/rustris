use crate::tetronimoe::{Figure, Direction};
use crate::game_state::{FIELD_WIDTH, FIELD_HEIGHT};

pub fn direction_is_clear(figure: &Figure, direction: Direction, field: &Vec<bool>) -> bool {
    let points = figure.tiles.clone();
    for point in points {
        match direction {
            Direction::Left => {
                if point.x == 0 {
                    return false;
                }

                if field[(point.y * FIELD_WIDTH as isize + point.x - 1) as usize] {
                    return false;
                }
            },
            Direction::Right => {
                if point.x == (FIELD_WIDTH - 1) as isize {
                    return false;
                }

                if field[(point.y * FIELD_WIDTH as isize + point.x + 1) as usize] {
                    return false;
                }
            },
            Direction::Down => {
                if point.y == (FIELD_HEIGHT - 1) as isize {
                    return false;
                }
                if field[((point.y + 1) * FIELD_WIDTH as isize + point.x) as usize] {
                    return false;
                }
            },
            _ => unreachable!(),
        }
    }

    return true;
}

pub fn position_is_clear(figure: &Figure, field: &Vec<bool>) -> bool {
    for point in figure.tiles.clone() {
        if field[(point.y * FIELD_WIDTH as isize + point.x) as usize] {
            return false;
        }
    }

    true
}