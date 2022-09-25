use sdl2;

pub struct Mouse {
    x_pos : f32,
    y_pos : f32,
    mouse_button : u8,
}

impl Mouse {
    pub fn create() -> Mouse{
    	Mouse {x_pos: 0.0, y_pos: 0.0, mouse_button : 0}
    }
    pub fn move_mouse(&mut self,x : f32, y : f32){
    	self.x_pos = x;
    	self.y_pos = y; 
    }
    pub fn get_x(&self) -> f32{
    	self.x_pos
    }
    pub fn get_y(&self) -> f32{
    	self.y_pos
    }
    pub fn get_mouse_button(&self) -> u8{
    	self.mouse_button
    }
    pub fn left_mouse_button_down(&mut self){
    	self.mouse_button += 1;
    }
    pub fn left_mouse_button_up(&mut self){
    	self.mouse_button -= 1;
    }
    pub fn right_mouse_button_down(&mut self){
    	self.mouse_button += 2;
    }
    pub fn right_mouse_button_up(&mut self){
    	self.mouse_button -= 2;
    }
    pub fn calculate_mouse_pos(&mut self, state : sdl2::mouse::MouseState,width : f32, height : f32){
    	self.move_mouse(state.x() as f32 / (width/2.0) -1.0,-1.0* (state.y() as f32 / (height/2.0) -1.0));
    }


}

impl Drop for Mouse {
    fn drop(&mut self) {
    }
}
