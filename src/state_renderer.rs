use crate::game_state::{GameState, FIELD_WIDTH, FIELD_HEIGHT};
use glium::{Surface, Display, Frame, VertexBuffer, IndexBuffer};
use nalgebra_glm as glm;
use glium_text_nxt::FontTexture;
use crate::tetronimoe::{BlockType, Point};
use crate::texture_bag::TextureBag;
use crate::shader_program::get_shader_program;
use glium::index::PrimitiveType::TrianglesList;
use crate::vertex::Vertex;
use std::ops::Deref;
use glium::texture::RawImage2d;
use nalgebra_glm::TVec3;

pub const CUP_COORDINATES_START_X: f32 = -0.6;
pub const CUP_COORDINATES_START_Y: f32 = 0.85;

pub fn render_sate(state: &GameState, display: &Display, target: &mut Frame, font: &FontTexture, textures: &TextureBag) {
    render_next_figure(state, display, target, font, textures);
    render_bucket(state, display, target, textures);
    render_controls_and_score(state, display, target, font);
    if state.is_paused {
        render_pause(display, target, font);
    }
}

fn render_next_figure(state: &GameState, display: &Display, target: &mut Frame, font: &FontTexture, textures: &TextureBag) {
    let text_system = glium_text_nxt::TextSystem::new(display);
    let text = glium_text_nxt::TextDisplay::new(&text_system, font, "Next figure");
    let mut score_matrix = glm::translate(&glm::identity(), &glm::vec3(-0.95, 0.9, 0.0));
    score_matrix = glm::scale(&score_matrix, &glm::vec3(0.04, 0.04, 0.04));
    glium_text_nxt::draw(&text, &text_system, target, score_matrix, (0.0, 0.0, 0.0, 1.0));

    let image = match state.next_figure.block_type {
        BlockType::Square => &textures.square,
        BlockType::LinePiece => &textures.line,
        BlockType::TBlock => &textures.tblock,
        BlockType::LBlock => &textures.lblock,
        BlockType::ReverseLBlock => &textures.reversed_lblock,
        BlockType::Squiggle => &textures.squiggle,
        BlockType::ReverseSquiggle => &textures.reversed_squiggle,
    };

    let shape = vec![
        Vertex {position: [-0.95, 0.85, 0.0], texture: [0.0, 1.0]}, // top-left
        Vertex {position: [-0.75, 0.85, 0.0], texture: [1.0, 1.0]}, // top-right
        Vertex {position: [-0.75, 0.7, 0.0], texture: [1.0, 0.0]}, // bottom-right
        Vertex {position: [-0.95, 0.7, 0.0], texture: [0.0, 0.0]}, // bottom-left
    ];

    let indices: [u16; 6] = [
        0, 1, 2,
        0, 3, 2
    ];

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        tex: image,
    };

    let vertex_buffer = VertexBuffer::new(display, &shape).unwrap();
    let index_buffer = IndexBuffer::new(display, TrianglesList, &indices).unwrap();
    target.draw(&vertex_buffer, &index_buffer, &get_shader_program(display), &uniforms, &Default::default())
        .unwrap();
}

fn render_bucket(state: &GameState, display: &Display, target: &mut Frame, textures: &TextureBag) {
    // render bucket itself
    let bucket_shape = vec![
        Vertex {position: [-0.6, 0.85, 0.0], texture: [0.0, 1.0]}, // top-left
        Vertex {position: [0.45, 0.85, 0.0], texture: [1.0, 1.0]}, // top-right
        Vertex {position: [0.45, -0.85, 0.0], texture: [1.0, 0.0]}, // bottom-right
        Vertex {position: [-0.6, -0.85, 0.0], texture: [0.0, 0.0]}, // bottom-left
    ];

    let indices: [u16; 6] = [
        0, 1, 2,
        0, 3, 2
    ];

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        tex: &textures.cup,
    };

    let vertex_buffer = VertexBuffer::new(display, &bucket_shape).unwrap();
    let index_buffer = IndexBuffer::new(display, TrianglesList, &indices).unwrap();
    target.draw(&vertex_buffer, &index_buffer, &get_shader_program(display), &uniforms, &Default::default())
        .unwrap();

    // render filled space
    for point in state.filled_space.clone() {
        render_point(point, display, target, textures);
    }

    // render current figure
    for point in state.current_figure.tiles.clone() {
        render_point(point, display, target, textures);
    }
}

fn render_point(point: Point, display: &Display, target: &mut Frame, textures: &TextureBag) {
    let point_width: f32 = 1.05 / FIELD_WIDTH as f32;
    let point_height: f32 = 1.7 / FIELD_HEIGHT as f32;

    let point_position_x: f32 = CUP_COORDINATES_START_X + point.x as f32 * point_width;
    let point_position_y: f32 = CUP_COORDINATES_START_Y - point.y as f32 * point_height;

    let point_shape = vec![
        Vertex {position: [point_position_x, point_position_y, 0.0], texture: [0.0, 1.0]}, // top-left
        Vertex {position: [point_position_x + point_width, point_position_y, 0.0], texture: [1.0, 1.0]}, // top-right
        Vertex {position: [point_position_x + point_width, point_position_y - point_height, 0.0], texture: [1.0, 0.0]}, // bottom-right
        Vertex {position: [point_position_x, point_position_y - point_height, 0.0], texture: [0.0, 0.0]}, // bottom-left
    ];

    let indices: [u16; 6] = [
        0, 1, 2,
        0, 3, 2
    ];

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        tex: &textures.block,
    };

    let vertex_buffer = VertexBuffer::new(display, &point_shape).unwrap();
    let index_buffer = IndexBuffer::new(display, TrianglesList, &indices).unwrap();
    target.draw(&vertex_buffer, &index_buffer, &get_shader_program(display), &uniforms, &Default::default())
        .unwrap();
}

fn render_controls_and_score(state: &GameState, display: &Display, target: &mut Frame, font: &FontTexture) {
    let score_text = format!("Score: {}", state.current_score);
    render_text(score_text, glm::vec3(0.5, 0.9, 0.0), display, target, font);

    let best_text = format!("Best: {}", state.best_score);
    render_text(best_text, glm::vec3(0.5, 0.8, 0.0), display, target, font);

    let level_text = format!("Level: {}", state.level);
    render_text(level_text, glm::vec3(0.5, 0.7, 0.0), display, target, font);

    let controls_header = String::from("Controls:");
    render_text(controls_header, glm::vec3(0.5, 0.4, 0.0), display, target, font);

    let controls_left_text = String::from("Left: Num4");
    render_text(controls_left_text, glm::vec3(0.5, 0.3, 0.0), display, target, font);

    let controls_right_text = String::from("Right: Num6");
    render_text(controls_right_text, glm::vec3(0.5, 0.2, 0.0), display, target, font);

    let controls_ccw_text = String::from("CCW: Num3|7");
    render_text(controls_ccw_text, glm::vec3(0.5, 0.1, 0.0), display, target, font);

    let controls_cw_text = String::from("CW: Num1|5|9");
    render_text(controls_cw_text, glm::vec3(0.5, 0.0, 0.0), display, target, font);

    let controls_hard_drop_text = String::from("Hard drop: Num8");
    render_text(controls_hard_drop_text, glm::vec3(0.5, -0.1, 0.0), display, target, font);

    let controls_soft_drop_text = String::from("Soft drop: Num2");
    render_text(controls_soft_drop_text, glm::vec3(0.5, -0.2, 0.0), display, target, font);

    let controls_pause_text = String::from("Pause: Esc|F1");
    render_text(controls_pause_text, glm::vec3(0.5, -0.3, 0.0), display, target, font);
}

fn render_pause(display: &Display, target: &mut Frame, font: &FontTexture) {
    let text_system = glium_text_nxt::TextSystem::new(display);
    let text = glium_text_nxt::TextDisplay::new(&text_system, font, "PAUSE");
    let scale_ratio = 2.0 * 0.7 / text.get_width();
    let mut text_matrix = glm::translate(&glm::identity(), &glm::vec3(-0.7, -0.1, 0.0));
    text_matrix = glm::scale(&text_matrix, &glm::vec3(scale_ratio, scale_ratio, scale_ratio));
    glium_text_nxt::draw(&text, &text_system, target, text_matrix, (0.0, 0.0, 0.0, 1.0));
}

fn render_text(text: String, position: TVec3<f32>, display: &Display, target: &mut Frame, font: &FontTexture) {
    let text_system = glium_text_nxt::TextSystem::new(display);
    let text_display = glium_text_nxt::TextDisplay::new(&text_system, font, text.as_str());
    let mut level_matrix = glm::translate(&glm::identity(), &position);
    level_matrix = glm::scale(&level_matrix, &glm::vec3(0.04, 0.04, 0.04));
    glium_text_nxt::draw(&text_display, &text_system, target, level_matrix, (0.0, 0.0, 0.0, 1.0));
}