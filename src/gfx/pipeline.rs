// wengwengweng

use super::*;

#[derive(Clone)]
pub(super) struct Pipeline<V: VertexLayout, U: UniformLayout> {
	gl: Rc<glow::Context>,
	gl_prog: Rc<ProgramHandle>,
	_vertex_layout: PhantomData<V>,
	_uniform_layout: PhantomData<U>,
}

impl<V: VertexLayout, U: UniformLayout> Pipeline<V, U> {

	pub fn new(ctx: &impl GLCtx, vert_src: &str, frag_src: &str) -> Result<Self> {

		unsafe {

			let gl = ctx.gl().clone();
			let gl_prog = ProgramHandle::new(&gl)?;

			let vert_id = gl.create_shader(ShaderType::Vertex.as_glow())?;

			gl.shader_source(vert_id, vert_src);
			gl.compile_shader(vert_id);

			if !gl.get_shader_compile_status(vert_id) {
				return Err(format!("vert error: {}", gl.get_shader_info_log(vert_id).trim()));
			}

			let frag_id = gl.create_shader(ShaderType::Fragment.as_glow())?;

			gl.shader_source(frag_id, frag_src);
			gl.compile_shader(frag_id);

			if !gl.get_shader_compile_status(frag_id) {
				return Err(format!("frag error: {}", gl.get_shader_info_log(frag_id).trim()));
			}

			gl.attach_shader(gl_prog.id(), vert_id);
			gl.attach_shader(gl_prog.id(), frag_id);

			for (i, (name, _)) in V::attrs().iter().enumerate() {
				gl.bind_attrib_location(gl_prog.id(), i as u32, name);
			}

			gl.link_program(gl_prog.id());

			if !gl.get_program_link_status(gl_prog.id()) {
				return Err(format!("glsl error: {}", gl.get_program_info_log(gl_prog.id()).trim()));
			}

			gl.delete_shader(vert_id);
			gl.delete_shader(frag_id);

			return Ok(Self {
				gl: gl,
				gl_prog: Rc::new(gl_prog),
				_vertex_layout: PhantomData,
				_uniform_layout: PhantomData,
			});

		}

	}

	pub(super) fn bind(&self) {
		unsafe {
			self.gl.use_program(Some(self.gl_prog.id()));
		}
	}

	pub(super) fn unbind(&self) {
		unsafe {
			self.gl.use_program(None);
		}
	}

	pub(super) fn loc(&self, name: &'static str) -> Option<glow::UniformLocation> {
		unsafe {
			return self.gl.get_uniform_location(self.gl_prog.id(), name);
		}
	}

}

impl<V: VertexLayout, U: UniformLayout> PartialEq for Pipeline<V, U> {
	fn eq(&self, other: &Self) -> bool {
		return self.gl_prog == other.gl_prog;
	}
}

