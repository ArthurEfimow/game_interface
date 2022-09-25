extern crate sdl2;
extern crate gl;
extern crate xml;
extern crate serde;
extern crate serde_json;
extern crate stb_image;
extern crate rand;

pub mod shader;
pub mod renderer;
pub mod mouse;
pub mod mesh;
pub mod form;

use std::ffi::{CString};
use crate::mouse::Mouse;
use std::collections::HashMap;

pub struct Durak {
    width  : f32,
    height : f32,
    mesh_ : mesh::Mesh,
    mouse : Mouse,
    helper : f32,
    _window : sdl2::video::Window,
    _gl_context : sdl2::video::GLContext,
    _gl : (),
    event_pump : sdl2::EventPump,
    textures: HashMap<String,u32>,
    forms: HashMap<u32,form::FormNT>,
    draw_order : Vec<u32>,
    forms_id : u32,
    form_texture_connection: HashMap<u32,String>,
}

impl Durak {

    pub fn create(width: f32, height: f32,name: &str) -> Durak{

    	let sdl = sdl2::init().unwrap();
    	let video_subsystem = sdl.video().unwrap();
    	let _window = video_subsystem
            .window(name, width as u32, height as u32)
	    .opengl()
            .resizable()
            .build()
            .unwrap();

    	let _gl_context = _window.gl_create_context().unwrap();

    	let gl_attr = video_subsystem.gl_attr();
    	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    	gl_attr.set_context_version(4, 5);

    	let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    	unsafe {
	    gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
    	}

    	let vert_shader = shader::Shader::from_vert_source(&CString::new(include_str!("../Shaders/triangle.vert")).unwrap()).unwrap();
    	let frag_shader = shader::Shader::from_frag_source(&CString::new(include_str!("../Shaders/triangle.frag")).unwrap()).unwrap();
    	let shader_program = renderer::Renderer::from_shaders(&[vert_shader, frag_shader]).unwrap();
    	shader_program.set_used();
    	
    	let event_pump = sdl.event_pump().unwrap();
    	
    	let mesh_ = mesh::Mesh::create();
    	
    	let textures: HashMap<String,u32> = HashMap::new();
    	let forms: HashMap<u32,form::FormNT> = HashMap::new();
    	let draw_order : Vec<u32> = vec![];
    	let forms_id = 0;
    	let mouse = Mouse::create();
    	let form_texture_connection: HashMap<u32,String> = HashMap::new();

     	Durak {width,height, helper : 0.0,mouse,_window,_gl_context,_gl,event_pump,mesh_,textures,forms,draw_order,forms_id,form_texture_connection}

    }
    pub fn run(&mut self) -> bool {

            for event in self.event_pump.poll_iter() {
            	match event {
                    sdl2::event::Event::Quit {..} => return false,
                    sdl2::event::Event::MouseButtonDown {mouse_btn :sdl2::mouse::MouseButton::Left, ..} =>  { self.mouse.left_mouse_button_down()},
	            sdl2::event::Event::MouseButtonUp { mouse_btn : sdl2::mouse::MouseButton::Left, ..} =>  { self.mouse.left_mouse_button_up()},
	            sdl2::event::Event::MouseButtonDown { mouse_btn : sdl2::mouse::MouseButton::Right, ..} => { self.mouse.right_mouse_button_down()},
                    sdl2::event::Event::MouseButtonUp { mouse_btn : sdl2::mouse::MouseButton::Right, ..} => { self.mouse.right_mouse_button_up()},
	            _ => {}
            	}	       
            }
           self.mouse.calculate_mouse_pos(self.event_pump.mouse_state(),self.width, self.height);
	   if self.mouse.get_mouse_button() > 0 {self.helper += 0.01;}
           self._window.gl_swap_window();
           unsafe {gl::ClearColor(0.0, 0.0, 0.0, 1.0);}
           unsafe {gl::Clear(gl::COLOR_BUFFER_BIT);}
           return true;
   }
   
   pub fn add_texture(&mut self,name : String,source : String){
    	self.textures.insert(name,create_texture(source));
   }
   
   pub fn draw_single(&self,form : u32,tex : String){
    	self.mesh_.draw(&self.forms.get(&form).unwrap().get_info(),*self.textures.get(&tex).unwrap());
    }
    
   pub fn draw(& self){
   	for index in &self.draw_order {
   	    match self.form_texture_connection.get(index){
   	        Some(meta) => self.draw_single(*index,meta.to_string()),
   	        None => println!("ERROR: Mesh {} has no texture",index),
   	    }
   	}
   }
    
   pub fn create_form(&mut self,x: f32, y :f32, l : f32, h :f32) -> u32{
   	self.forms_id+=1;
    	self.forms.insert(self.forms_id,form::FormNT::create_qua_sta(x,y,l,h));
    	self.draw_order.push(self.forms_id);
    	self.forms_id
   }
   
   pub fn connect_tex_form(&mut self,tex : String, form : u32){
   	self.form_texture_connection.insert(form,tex);
   }
}

impl Drop for Durak {
    fn drop(&mut self) {
        for (_, tex) in &self.textures {drop_texture(*tex);}
    }
}

    pub fn create_texture(source : String) -> u32 {
    	let mut width  : i32 = 0;
    	let mut height : i32 = 0;
    	let mut nr_channels : i32 = 0;
    	let mut texture :u32 = 0;
    	unsafe {
	    gl::GenTextures(1, &mut texture as *mut u32);
	    gl::BindTexture(gl::TEXTURE_2D, texture);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
	    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

	    let data = stb_image::stb_image::bindgen::stbi_load(CString::new(source).unwrap().as_ptr() as *const i8,&mut width as *mut i32, &mut height as *mut i32, &mut nr_channels as *mut i32, 4);
	
	    if !data.is_null() {
		gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA as u32, gl::UNSIGNED_BYTE, data as *const std::ffi::c_void);
    		gl::GenerateMipmap(gl::TEXTURE_2D);
	    }
	    stb_image::stb_image::bindgen::stbi_image_free(data as *mut std::ffi::c_void);

    	}

	return texture;
    }
    
fn drop_texture(texture :u32) {
	unsafe {
	    gl::DeleteTextures(0, texture as * mut u32);	
	}
}
