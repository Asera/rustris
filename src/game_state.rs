use crate::tetronimoe::{Figure, Point, Direction};
use crate::collision_checker::{direction_is_clear, position_is_clear};
use nalgebra::max;

pub const FIELD_WIDTH : u32 = 10;
pub const FIELD_HEIGHT : u32 = 22;

// Typical game state
pub struct GameState {
    pub is_paused: bool,
    pub current_score: u64,
    pub best_score: u64,
    pub level: u32,
    pub progress_to_next_level: u32,
    pub filled_space: Vec<Point>, // todo remake, use something to count coordinates via formulae
        // or use two-dimensional array
    pub current_figure: Figure,
    pub next_figure: Figure,
    pub gravity: f64,
    pub next_gravity_tick: i32,
}

impl GameState {
    pub fn init() -> GameState {
        GameState {
            is_paused: false,
            current_score: 0,
            best_score: 0,
            level: 1,
            progress_to_next_level: 0,
            filled_space: vec![],
            current_figure: Figure::init_random(),
            next_figure: Figure::init_random(),
            gravity: GameState::get_level_gravity(1),
            next_gravity_tick: (1.0 / GameState::get_level_gravity(1)) as i32,
        }
    }

    pub fn restart(&mut self) {
        if self.current_score > self.best_score {
            self.best_score = self.current_score;
        }

        self.current_score = 0;
        self.level = 1;
        self.progress_to_next_level = 0;
        self.filled_space = vec![];
        self.current_figure = Figure::init_random();
        self.next_figure = Figure::init_random();
        self.gravity = GameState::get_level_gravity(1);
        self.next_gravity_tick = (1.0 / GameState::get_level_gravity(1)) as i32;
    }

    pub fn update(&mut self) {
        if self.is_paused {
            return ;
        }

        if direction_is_clear(&self.current_figure, Direction::Down, &self.get_field_as_array()) {
            if self.next_gravity_tick <= 0 {
                self.current_figure.shift_down();
                self.next_gravity_tick = (1.0 / self.gravity) as i32
            } else {
                self.next_gravity_tick -= 1;
            }
        } else {
            self.filled_space.append(&mut self.current_figure.tiles.clone());
            self.current_figure = self.next_figure.clone();
            self.next_figure = Figure::init_random();
        }

        let cleared_lines_count = self.count_filled_lines();
        self.clear_filled_lines();
        self.update_score(cleared_lines_count);
        self.update_level_progress(cleared_lines_count);
    }

    pub fn game_is_finished(&self) -> bool {
        let start_position = Figure::get_start_position();
        !position_is_clear(&self.current_figure, &self.get_field_as_array())
            && self.current_figure.center.x == start_position.x
            && self.current_figure.center.y == start_position.y
    }

    pub fn rotate_clockwise(&mut self) {
        let mut possible_position = self.current_figure.clone();
        possible_position.rotate_cw();
        match self.resolve_rotation_collision(possible_position) {
            None => (),
            Some(possible_position) => {
                self.current_figure = possible_position;
            }
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        let mut possible_position = self.current_figure.clone();
        possible_position.rotate_ccw();
        match self.resolve_rotation_collision(possible_position) {
            None => (),
            Some(possible_position) => {
                self.current_figure = possible_position;
            }
        }
    }

    fn resolve_rotation_collision(&self, mut figure_position: Figure) -> Option<Figure> {
        let points = figure_position.tiles.clone();
        // check top left point
        let mut top_left_point = points.first().unwrap().clone();
        let mut top_right_point = points.first().unwrap().clone();
        // check top right point
        for point in points {
            // lets just assume that we can't rotate figure if it is too low
            if point.y < 0 {
                return None;
            }

            if point.x < top_left_point.x {
                top_left_point = point;
            } else if point.x > top_right_point.x {
                top_right_point = point;
            }
        }

        // right shift on two blocks
        if top_left_point.x == -2 {
            if direction_is_clear(&figure_position, Direction::Right, &self.get_field_as_array()) {
                figure_position.shift_right();
            } else {
                return None;
            }
            return if direction_is_clear(&figure_position, Direction::Right, &self.get_field_as_array()) {
                figure_position.shift_right();
                Some(figure_position)
            } else {
                None
            }
        }

        // right shift on one block
        if top_left_point.x == -1 {
            return if direction_is_clear(&figure_position, Direction::Right, &self.get_field_as_array()) {
                figure_position.shift_right();
                Some(figure_position)
            } else {
                None
            }
        }

        // left shift on one block
        if top_right_point.x == FIELD_WIDTH as isize {
            return if direction_is_clear(&figure_position, Direction::Left, &self.get_field_as_array()) {
                figure_position.shift_left();
                Some(figure_position)
            } else {
                None
            }
        }

        // left shift on two block
        if top_right_point.x == (FIELD_WIDTH + 1) as isize {
            if direction_is_clear(&figure_position, Direction::Left, &self.get_field_as_array()) {
                figure_position.shift_left();
            } else {
                return None;
            }

            return if direction_is_clear(&figure_position, Direction::Left, &self.get_field_as_array()) {
                figure_position.shift_left();
                Some(figure_position)
            } else {
                None
            }
        }

        return Some(figure_position);
    }

    // Hard drop is 60 blocks per second.
    // I know that it is incorrect, and it must be 20 blocks per frame.
    // I'll fix it in the future, probably.
    // Or not. I'm making this as an experiment, I can play with mechanics.
    pub fn set_hard_drop_gravity(&mut self) {
        self.gravity = 20.0;
        self.next_gravity_tick = 0;
    }

    // Soft drop speed is 20 blocks per second
    pub fn set_soft_drop_gravity(&mut self) {
        self.gravity = 0.3;
        self.next_gravity_tick = 0;
    }

    pub fn reset_gravity(&mut self) {
        self.gravity = GameState::get_level_gravity(self.level);
        self.next_gravity_tick = (1.0 / GameState::get_level_gravity(self.level)) as i32;
    }

    fn get_level_gravity(level: u32) -> f64 {
        ((0.8 - ((level as f32 - 1.0) * 0.007)).powf((level - 1) as f32)) as f64 / 60.0
    }

    pub fn left_shift(&mut self) {
        if direction_is_clear(&self.current_figure, Direction::Left, &self.get_field_as_array()) {
            self.current_figure.shift_left();
        }
    }

    pub fn right_shift(&mut self) {
        if direction_is_clear(&self.current_figure, Direction::Right, &self.get_field_as_array()) {
            self.current_figure.shift_right();
        }
    }

    pub fn pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn hold(&self) { //todo implement in later versions
        unimplemented!()
    }

    fn count_filled_lines(&self) -> u32 {
        let field_as_array = self.get_field_as_array();

        let mut result = 0;
        for i in 0..FIELD_HEIGHT {
            let mut line_full = true;
            for j in 0..FIELD_WIDTH {
                line_full = line_full && field_as_array[(j + i * FIELD_WIDTH) as usize];
            }
            if line_full {
                result += 1;
            }
        }

        return result;
    }

    fn clear_filled_lines(&mut self) {
        // transform points to array
        let mut field_as_array = self.get_field_as_array();
        // go from top to bottom: y 0 -> 22
        for i in 0..FIELD_HEIGHT {
            let mut line_filled = true;
            for j in 0..FIELD_WIDTH {
                line_filled = line_filled && field_as_array[(j + i * FIELD_WIDTH) as usize];
            }
            // if line is fully filled
            if line_filled {
                // all lines above are shifted down
                for k in (0..i+1).rev() {
                    for l in 0..FIELD_WIDTH {
                        if k == 0 {
                            field_as_array[(l + k * FIELD_WIDTH) as usize] = false;
                        } else {
                            field_as_array[(l + k * FIELD_WIDTH) as usize] = field_as_array[(l + (k - 1) * FIELD_WIDTH) as usize];
                        }
                    }
                }
            }
        }

        self.store_field_from_array(field_as_array);
    }

    // true - cell is occupied with block
    // false - cell is empty
    pub fn get_field_as_array(&self) -> Vec<bool> {
        let field_size = (FIELD_HEIGHT * FIELD_WIDTH) as usize;
        // Field represented in consecutive rows: row0, row1,...
        // Position:
        //  - Point to array: index = Point.x + Point.y * FIELD_WIDTH
        //  - Array to point: Point {x: index % FIELD_WIDTH, y: index / FIELD_WIDTH
        let mut field_as_array: Vec<bool> = vec![false; field_size];

        for point in &self.filled_space {
            field_as_array[(point.x + point.y * FIELD_WIDTH as isize) as usize] = true;
        }

        return field_as_array;
    }

    pub fn store_field_from_array(&mut self, array_field: Vec<bool>) {
        let mut field_as_points = vec![];
        for (index, element) in array_field.iter().enumerate() {
            if *element {
                field_as_points.push(Point {x: index as isize % FIELD_WIDTH as isize, y: index as isize / FIELD_WIDTH as isize});
            }
        }

        self.filled_space = field_as_points;
    }

    fn update_score(&mut self, cleared_lines_count: u32) {
        let score_base: u64 = match cleared_lines_count {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            _ => 1200,
        };

        self.current_score += score_base * (self.level + 1) as u64;
    }

    fn update_level_progress(&mut self, cleared_lines_count: u32) {
        self.progress_to_next_level += cleared_lines_count;

        if self.progress_to_next_level >= (self.level * 5) {
            self.progress_to_next_level -= self.level * 5;
            self.level += 1;
        }
    }
}