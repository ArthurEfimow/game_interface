pub struct Mesh {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
}

impl Mesh {
    pub fn create() -> Mesh {
    	let mut vbo: gl::types::GLuint = 0;
	let mut vao: gl::types::GLuint = 0;
	
    	unsafe {
    	    gl::GenBuffers(1, &mut vbo);
    	    gl::GenVertexArrays(1, &mut vao);
 	    gl::BindVertexArray(vao);
	    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
	    gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    	    gl::VertexAttribPointer(
            	0, // index of the generic vertex attribute ("layout (location = 0)")
            	2, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	std::ptr::null() // offset of the first component
    	    );
    	    gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
    	    gl::VertexAttribPointer(
            	1, // index of the generic vertex attribute ("layout (location = 1)")
            	4, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	(2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
            );
    	    gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
    	    gl::VertexAttribPointer(
            	2, // index of the generic vertex attribute ("layout (location = 2)")
            	2, // the number of components per generic vertex attribute
            	gl::FLOAT, // data type
            	gl::FALSE, // normalized (int-to-float conversion)
            	(8 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            	(6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
            );
    	}
	return Mesh {vao,vbo};
    }

    pub fn draw(&self,vertices: &Vec<f32>,tex : u32){
        unsafe {
	    gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
    	    gl::BufferData(
            gl::ARRAY_BUFFER, // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
    	    );
      	    gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
	    bind_texture(tex);
	    gl::BindVertexArray(self.vao);
	    gl::DrawArrays(gl::TRIANGLES,0,vertices.len() as i32);
	    gl::BindVertexArray(0);
        }
    }
    
    
}

impl Drop for Mesh {
    fn drop(&mut self) {
	unsafe {
	    gl::DeleteVertexArrays(0,  self.vao as * mut u32 );
	    gl::DeleteBuffers(0, self.vbo as * mut u32);
	}
    }
}

pub fn bind_texture(texture :u32){
        unsafe {
	    gl::ActiveTexture(gl::TEXTURE0);
	    gl::BindTexture(gl::TEXTURE_2D,texture);
        }
}


