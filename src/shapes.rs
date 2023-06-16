
use glow::HasContext;

//pub struct Quad {
//    mesh: Mesh
//}
//
//impl Quad {
//    pub fn new(height: u32, width: u32) -> Quad {
//
//    }
//}

pub struct Circle {
    vbo: glow::Buffer,
    vao: glow::VertexArray,
    shader: glow::Program,
    vertices: usize,

    center: nalgebra_glm::Vec2,
    radius: f32
}

impl Circle {
    pub fn new(gl: &glow::Context, center: nalgebra_glm::Vec2, radius: f32) -> Self {
        unsafe {
            let prog = gl.create_program().expect("Cannot create prog");
            let shaders = vec![
                (glow::VERTEX_SHADER, include_str!("../shaders/circle.vert")),
                (
                    glow::FRAGMENT_SHADER,
                    include_str!("../shaders/circle.frag"),
                ),
            ];

            for (tp, source) in shaders.iter() {
                let s = gl.create_shader(*tp).expect("oh oh");
                gl.shader_source(s, source);
                gl.compile_shader(s);
                if !gl.get_shader_compile_status(s) {
                    let log = gl.get_shader_info_log(s);
                    println!("Error: {}", log);
                }
                gl.attach_shader(prog, s);
            }

            gl.link_program(prog);
            if !gl.get_program_link_status(prog) {
                let log = gl.get_program_info_log(prog);
                println!("Error: {}", log);
            }
            gl.use_program(Some(prog));

            let vbo = gl.create_buffer().expect("Cannot create buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));


            let mut angle = 0.0f32;
            let mut verts = Vec::<f32>::new();
            while angle < f32::to_radians(360.0) {
                verts.push(center.x + radius * f32::cos(angle));
                verts.push(center.y + radius * f32::sin(angle));
                angle += f32::to_radians(0.5);
            }

            let vert_count = verts.len() / 2;

            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, verts.align_to().1, glow::STATIC_DRAW);

            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vao));

            let position_location = gl.get_attrib_location(prog, "position").unwrap();
            gl.enable_vertex_attrib_array(position_location);
            gl.vertex_attrib_pointer_f32(position_location, 2, glow::FLOAT, false, 8, 0);

            //let tex_location = gl.get_attrib_location(prog, "tex").unwrap();
            //gl.enable_vertex_attrib_array(tex_location);
            //gl.vertex_attrib_pointer_f32(tex_location, 2, glow::FLOAT, false, 8, 8);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            Self {
                vbo,
                vao,
                shader: prog,
                vertices: vert_count,
                center,
                radius
            }
        }
    }

    pub fn render(&mut self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.shader));

            //let old_buffer =
            //    std::num::NonZeroU32::new(gl.get_parameter_i32(glow::ARRAY_BUFFER_BINDING) as u32)
            //        .map(glow::NativeBuffer);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

            //let old_vao =
            //    std::num::NonZeroU32::new(gl.get_parameter_i32(glow::VERTEX_ARRAY_BINDING) as u32)
            //        .map(glow::NativeVertexArray);

            gl.bind_vertex_array(Some(self.vao));

            //let elapsed = self.start_time.elapsed().as_millis() as f32;
            //gl.uniform_1_f32(Some(&self.effect_time_location), elapsed);
            //gl.uniform_1_f32(
            //    Some(&self.rotation_time_location),
            //    if rotation_enabled { elapsed } else { 0.0 },
            //);

            gl.line_width(3.0);
            gl.draw_arrays(glow::TRIANGLE_FAN, 0, self.vertices.try_into().unwrap());

            //gl.bind_buffer(glow::ARRAY_BUFFER, old_buffer);
            //gl.bind_vertex_array(old_vao);
            gl.use_program(None);
        }
    }
}
