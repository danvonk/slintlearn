mod mesh;
mod shapes;

use glow::HasContext;
use nalgebra_glm::Vec2;

use shapes::Circle;
use slint::private_unstable_api::re_exports::PointerEventKind;

struct EGLUnderlay {
    gl: glow::Context,
    program: glow::Program,
    effect_time_location: glow::UniformLocation,
    rotation_time_location: glow::UniformLocation,
    vbo: glow::Buffer,
    vao: glow::VertexArray,
    start_time: std::time::Instant,
    circle: Circle,
}

impl EGLUnderlay {
    fn new(gl: glow::Context) -> Self {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                include_str!("../shaders/tut.vert"),
                include_str!("../shaders/tut.frag"),
            );

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let effect_time_location = gl.get_uniform_location(program, "effect_time").unwrap();
            let rotation_time_location = gl.get_uniform_location(program, "rotation_time").unwrap();
            let position_location = gl.get_attrib_location(program, "position").unwrap();

            let vbo = gl.create_buffer().expect("Cannot create buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let vertices = [
                -1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32, 1.0f32, 1.0f32, -1.0f32,
            ];

            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices.align_to().1, glow::STATIC_DRAW);

            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vao));
            gl.enable_vertex_attrib_array(position_location);
            gl.vertex_attrib_pointer_f32(position_location, 2, glow::FLOAT, false, 8, 0);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            let circle = Circle::new(&gl, Vec2::new(-0.5, -0.5), 0.1);

            Self {
                gl,
                program,
                effect_time_location,
                rotation_time_location,
                vbo,
                vao,
                start_time: std::time::Instant::now(),
                circle,
            }
        }
    }
}

impl Drop for EGLUnderlay {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
            self.gl.delete_vertex_array(self.vao);
            self.gl.delete_buffer(self.vbo);
        }
    }
}

impl EGLUnderlay {
    fn render(&mut self, rotation_enabled: bool) {
        unsafe {
            let gl = &self.gl;

            gl.use_program(Some(self.program));

            let old_buffer =
                std::num::NonZeroU32::new(gl.get_parameter_i32(glow::ARRAY_BUFFER_BINDING) as u32)
                    .map(glow::NativeBuffer);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

            let old_vao =
                std::num::NonZeroU32::new(gl.get_parameter_i32(glow::VERTEX_ARRAY_BINDING) as u32)
                    .map(glow::NativeVertexArray);

            gl.bind_vertex_array(Some(self.vao));

            let elapsed = self.start_time.elapsed().as_millis() as f32;
            gl.uniform_1_f32(Some(&self.effect_time_location), elapsed);
            gl.uniform_1_f32(
                Some(&self.rotation_time_location),
                if rotation_enabled { elapsed } else { 0.0 },
            );

            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

            gl.bind_buffer(glow::ARRAY_BUFFER, old_buffer);
            gl.bind_vertex_array(old_vao);
            gl.use_program(None);

            self.circle.render(gl);
        }
    }
}

pub fn main() {
    let app = MainWindow::new().unwrap();

    let mut underlay = None;

    let app_weak = app.as_weak();

    app.on_pointer_event(|event| match event.kind {
        PointerEventKind::Down => {
            println!("Mouse down");
        }
        PointerEventKind::Up => {
            println!("Mouse up");
        }
        _ => {}
    });

    if let Err(error) =
        app.window()
            .set_rendering_notifier(move |state, graphics_api| match state {
                slint::RenderingState::RenderingSetup => {
                    let context = match graphics_api {
                        slint::GraphicsAPI::NativeOpenGL { get_proc_address } => unsafe {
                            glow::Context::from_loader_function_cstr(|s| get_proc_address(s))
                        },
                        _ => return,
                    };
                    underlay = Some(EGLUnderlay::new(context))
                }
                slint::RenderingState::BeforeRendering => {
                    if let (Some(underlay), Some(app)) = (underlay.as_mut(), app_weak.upgrade()) {
                        underlay.render(true);
                        app.window().request_redraw();
                    }
                }
                slint::RenderingState::AfterRendering => {}
                slint::RenderingState::RenderingTeardown => {
                    drop(underlay.take());
                }
                _ => {}
            })
    {
        match error {
            slint::SetRenderingNotifierError::Unsupported => eprintln!("This example requires the use of the GL backend. Please run with the environment variable SLINT_BACKEND=GL set."),
            _ => unreachable!()
        }
        std::process::exit(1);
    }

    app.run().unwrap();
}

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, CheckBox } from "std-widgets.slint";

    export component MainWindow inherits Window {

    preferred-width: 800px;
    preferred-height: 600px;
    title: "Slint OpenGL Underlay Example";

    in property <bool> rotation-enabled <=> apply-rotation.checked;

    callback clicked();
    callback pointer_event(PointerEvent);


    VerticalBox {
        Rectangle {
            background: #ffffff92;
            HorizontalBox {
                Text {
                    text: "This text is rendered using Slint. The animation below is rendered using OpenGL code.";
                    wrap: word-wrap;
                }

                VerticalLayout {
                    alignment: start;
                    apply-rotation := CheckBox {
                        checked: true;
                        text: "Rotate the OpenGL underlay";
                    }
                }
            }
        }
        Rectangle {
            area := TouchArea {
                //clicked => {
                //    clicked();
                //}
                pointer-event(e) => {
                    root.pointer_event(e);
                }
            }
 // A radius of width/2 makes it a circle
    Rectangle {
        x: 210px;
        y: 10px;
        width: 50px;
        height: 50px;
        background: yellow;
        border-width: 2px;
        border-color: blue;
        border-radius: self.width/2;
    }
        } // fill the rest of the screen
    }
}
}
