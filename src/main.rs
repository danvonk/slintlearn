mod mesh;
mod shapes;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use glow::HasContext;
use nalgebra_glm::Vec2;
use shapes::Circle;
use slint::{
    platform::PointerEventButton, private_unstable_api::re_exports::PointerEventKind, Model,
};
use std::{collections::HashMap, rc::Rc};

struct EGLUnderlay {
    gl: glow::Context,
    program: glow::Program,
    uniform_locs: HashMap<String, glow::UniformLocation>,
    vbo: glow::Buffer,
    vao: glow::VertexArray,
    start_time: std::time::Instant,
    window_x: f32,
    window_y: f32,
    points: Rc<slint::VecModel<Circ>>,
}

impl EGLUnderlay {
    fn new(gl: glow::Context, w: f32, h: f32, pts: Rc<slint::VecModel<Circ>>) -> Self {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let shader_sources = [
                (glow::VERTEX_SHADER, include_str!("../shaders/tut.vert")),
                (glow::FRAGMENT_SHADER, include_str!("../shaders/tut.frag")),
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

            let loc_map = HashMap::<String, glow::UniformLocation>::from([
                (
                    "effect_time".into(),
                    gl.get_uniform_location(program, "effect_time").unwrap(),
                ),
                (
                    "resolution".into(),
                    gl.get_uniform_location(program, "resolution").unwrap(),
                ),
                (
                    "cool_colours".into(),
                    gl.get_uniform_location(program, "cool_colours").unwrap(),
                ),
                (
                    "points".into(),
                    gl.get_uniform_location(program, "points").unwrap(),
                ),
                (
                    "has_points".into(),
                    gl.get_uniform_location(program, "has_points").unwrap(),
                ),
            ]);

            let vbo = gl.create_buffer().expect("Cannot create buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let vertices = [
                -1.0f32, 1.0f32, -1.0f32, -1.0f32, 1.0f32, 1.0f32, 1.0f32, -1.0f32,
            ];
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices.align_to().1, glow::STATIC_DRAW);

            let position_location = gl.get_attrib_location(program, "position").unwrap();
            let vao = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vao));
            gl.enable_vertex_attrib_array(position_location);
            gl.vertex_attrib_pointer_f32(position_location, 2, glow::FLOAT, false, 8, 0);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            Self {
                gl,
                program,
                uniform_locs: loc_map,
                vbo,
                vao,
                start_time: std::time::Instant::now(),
                window_x: w,
                window_y: h,
                points: pts,
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
    fn render(&mut self, cool_colours: bool) {
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

            let elapsed = self.start_time.elapsed().as_millis() as f32 / 1000.0;

            gl.uniform_1_f32(Some(&self.uniform_locs["effect_time"]), elapsed);
            gl.uniform_1_i32(
                Some(&self.uniform_locs["cool_colours"]),
                if cool_colours { 1 } else { 0 },
            );
            gl.uniform_1_i32(
                Some(&self.uniform_locs["has_points"]),
                if self.points.row_count() >= 1 { 1 } else { 0 },
            );
            gl.uniform_2_f32(
                Some(&self.uniform_locs["resolution"]),
                self.window_x,
                self.window_y,
            );

            let points_vec = match self.points.row_count() {
                1 => {
                    let p = self.points.row_data(0).unwrap();
                    nalgebra_glm::Vec4::new(
                        p.pos_x,
                        f32::max(0.0, self.window_y - p.pos_y),
                        0.0,
                        0.0,
                    )
                }
                2 => {
                    let p = self.points.row_data(0).unwrap();
                    let q = self.points.row_data(1).unwrap();
                    nalgebra_glm::Vec4::new(
                        p.pos_x,
                        f32::max(0.0, self.window_y - p.pos_y),
                        q.pos_x,
                        f32::max(0.0, self.window_y - q.pos_y),
                    )
                }
                _ => nalgebra_glm::Vec4::new(0.0, 0.0, 0.0, 0.0),
            };
            gl.uniform_4_f32_slice(Some(&self.uniform_locs["points"]), points_vec.as_slice());

            println!("Points is {}", points_vec);

            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

            gl.bind_buffer(glow::ARRAY_BUFFER, old_buffer);
            gl.bind_vertex_array(old_vao);
            gl.use_program(None);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let app = MainWindow::new().unwrap();

    let mut underlay = None;

    let app_weak = app.as_weak();

    let circs = Rc::new(slint::VecModel::<Circ>::from(vec![]));
    app.set_points(circs.clone().into());

    let circ_clone = circs.clone();

    app.on_add_point(move |event, mouse_x, mouse_y| match event.button {
        PointerEventButton::Left => match event.kind {
            PointerEventKind::Up => {
                let circs_model = circs.clone();
                if circs_model.row_count() < 2 {
                    println!("Mouse is at ({},{})", mouse_x, mouse_y);
                    circs_model.push(Circ {
                        pos_x: mouse_x,
                        pos_y: mouse_y,
                    });
                }
            }
            _ => {}
        },
        _ => {}
    });

    if let Err(error) =
        app.window()
            .set_rendering_notifier(move |state, graphics_api| match state {
                slint::RenderingState::RenderingSetup => {
                    let context = match graphics_api {
                        #[cfg(not(target_arch = "wasm32"))]
                        slint::GraphicsAPI::NativeOpenGL { get_proc_address } => unsafe {
                            glow::Context::from_loader_function_cstr(|s| get_proc_address(s))
                        },
                        #[cfg(target_arch = "wasm32")]
                        slint::GraphicsAPI::WebGL {
                            canvas_element_id,
                            context_type,
                        } => {
                            use wasm_bindgen::JsCast;

                            let canvas = web_sys::window()
                                .unwrap()
                                .document()
                                .unwrap()
                                .get_element_by_id(canvas_element_id)
                                .unwrap()
                                .dyn_into::<web_sys::HtmlCanvasElement>()
                                .unwrap();

                            let webgl1_context = canvas
                                .get_context(context_type)
                                .unwrap()
                                .unwrap()
                                .dyn_into::<web_sys::WebGlRenderingContext>()
                                .unwrap();

                            glow::Context::from_webgl1_context(webgl1_context)
                        }
                        _ => return,
                    };
                    unsafe {
                        context.viewport(0, 0, 800, 600);
                    }
                    underlay = Some(EGLUnderlay::new(context, 800.0, 600.0, circ_clone.clone()));
                }
                slint::RenderingState::BeforeRendering => {
                    if let (Some(underlay), Some(app)) = (underlay.as_mut(), app_weak.upgrade()) {
                        underlay.render(app.get_colours_enabled());
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

    let window_size = app.window().size();
    println!(
        "Window size is {} {}",
        window_size.width, window_size.height
    );

    app.run().unwrap();
}

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, CheckBox } from "std-widgets.slint";

    export struct Circ {
        pos-x: length,
        pos-y: length,
    }

    export component MainWindow inherits Window {

    width: 800px;
    height: 600px;
    title: "Slint OpenGL Underlay Example";

    in property <bool> colours-enabled <=> apply-colours.checked;
    in property <[Circ]> points;


    callback clicked();
    callback add_point(PointerEvent, length, length);

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
                    apply-colours := CheckBox {
                        checked: false;
                        text: "Cool Colours";
                    }
                }
            }
        }
        Rectangle {
            // rectangle fills the rest of the screen
            area := TouchArea {
                pointer-event(e) => {
                    root.add_point(e, self.mouse-x, self.mouse-y);
                }
            }
            for r in root.points : Rectangle {
                x: r.pos-x;
                y: r.pos-y;
                width: 20px;
                height: 20px;
                background: yellow;
                border-width: 2px;
                border-color: blue;
                border-radius: self.width/2;
            }
        }
    }
}
}
