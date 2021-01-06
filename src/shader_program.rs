use glium::{Display, Program};

pub fn get_shader_program(display: &Display) -> Program {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec2 texture;

        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = texture;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;

        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}