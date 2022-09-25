use gl;
use std;


use crate::shader;

pub struct Renderer {
    id: gl::types::GLuint,
}

impl Renderer {
    pub fn from_shaders(shaders: &[shader::Shader]) -> Result<Renderer, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        unsafe { gl::LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
	unsafe {
    	    gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
	}

	if success == 0 {
    	    let mut len: gl::types::GLint = 0;
    	    unsafe {
        	gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = shader::create_whitespace_cstring_with_len(len as usize);

 	    unsafe {
        	gl::GetProgramInfoLog(
            	    program_id,
            	    len,
            	    std::ptr::null_mut(),
            	    error.as_ptr() as *mut gl::types::GLchar
        	);
    	    }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Renderer { id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
    	unsafe {
            gl::UseProgram(self.id);
    	}
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}


