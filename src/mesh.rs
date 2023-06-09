
struct Mesh {
    vbo: glow::Buffer,
    vao: glow::VertexArray,
    vert_count: usize
}

impl Mesh {
    fn new(gl: &glow::Context) -> Self {
        Mesh {}
    }
}
