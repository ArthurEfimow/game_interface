pub struct Background {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

#[derive(Copy, Clone)]
pub struct Point{
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub s: f32,
    pub t: f32
}

impl Point {
    pub fn create_point(x: f32,y:f32,r: f32,g:f32,b:f32,a:f32,s:f32,t:f32 ) -> Point {
	return Point {x,y,r,g,b,a,s,t};
    }
    pub fn get_x(&self) -> f32 {return self.x;}
    pub fn get_y(&self) -> f32 {return self.y;}
    pub fn set_x (&mut self,pos : f32) { self.x = pos;}
    pub fn set_y (&mut self,pos : f32) { self.y = pos;}
    pub fn set_xy (&mut self,x : f32,y : f32) {self.x = x;self.y = y;}
    pub fn move_x (&mut self,length : f32) { self.x += length;}
    pub fn move_y (&mut self,length : f32) { self.y += length;}
    pub fn move_xy (&mut self,length_x : f32,length_y : f32) {self.x += length_x;self.y += length_y;}

    pub fn get_info (&self) -> Vec<f32> {return vec![self.x,self.y,self.r,self.g,self.b,self.a,self.s,self.t];}
    pub fn get_info_add (&self,add:Point) -> Vec<f32> {return vec![self.x+add.x,self.y+add.y,self.r+add.r,self.g+add.g,self.b+add.b,self.a+add.a,self.s+add.s,self.t+add.t];}

    pub fn get_copy (&self) -> &Point {return self;}  

    pub fn get_degree_to_point (&self,x: f32,y :f32) -> f32 {
	let mx = self.x;
	let my = self.y;

	let distance = ((x - mx) * (x - mx) + (y - my) * (y - my)).sqrt();
	let alpha_sin = ((y - my)/distance).asin().to_degrees();
        let alpha_cos = ((x - mx)/distance).acos().to_degrees();
	if alpha_sin as i32 == alpha_cos as i32 {return 90.0 - alpha_sin;}
	else if -1*alpha_sin as i32 == alpha_cos as i32 {return 90.0 - alpha_sin;}
	else if alpha_sin <= 0.0 {return 90.0 + alpha_cos;}
	else {return 270.0 + alpha_sin;}
    }

    pub fn move_degree_around_point (&mut self,deg: f32,x: f32,y :f32) {
	let mx = self.x;
	let my = self.y;

	let distance = ((mx - x) * (mx - x) + (my - y) * (my - y)).sqrt();
	let alpha_sin = ((my - y)/distance).asin().to_degrees();
        let alpha_cos = ((mx - x)/distance).acos().to_degrees();
	let mut org_deg;

	if alpha_sin as i32 == alpha_cos as i32 {org_deg = 90.0 - alpha_sin;}
	else if -1*alpha_sin as i32 == alpha_cos as i32 {org_deg = 90.0 - alpha_sin;}
	else if  alpha_sin <= 0.0 {org_deg = 90.0 + alpha_cos;}
	else {org_deg = 270.0 + alpha_sin;}

	org_deg += deg;
	loop {if org_deg < 360.0 {break;};org_deg -= 360.0;}
        loop {if org_deg >= 0.0 {break;};org_deg += 360.0;}

	self.x = x + org_deg.to_radians().sin() * distance;
	self.y = y + org_deg.to_radians().cos() * distance;
	
    }

}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
	return Point{x: self.x+_rhs.x,y: self.y+_rhs.y,r: self.r+_rhs.r,g:self.g+_rhs.g,b:self.b+_rhs.b,a:self.a+_rhs.a,s:self.s+_rhs.s,t:self.t+_rhs.t}
    }
}

#[derive(Clone)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point
}

impl Triangle {
    pub fn create(a : Point, b : Point, c : Point) -> Triangle {
	return Triangle {a,b,c};
    }
    pub fn move_x (&mut self,length : f32) { self.a.move_x(length);self.b.move_x(length);self.c.move_x(length);}
    pub fn move_y (&mut self,length : f32) { self.a.move_y(length);self.b.move_y(length);self.c.move_y(length);}
    
    pub fn set_x (&mut self,length : f32) { self.a.set_x(length);self.b.set_x(length);self.c.set_x(length);}
    pub fn set_y (&mut self,length : f32) { self.a.set_y(length);self.b.set_y(length);self.c.set_y(length);}

    pub fn get_info (&self, ret : &mut Vec<f32>) { 
	ret.extend(self.a.get_info().iter());
	ret.extend(self.b.get_info().iter());
	ret.extend(self.c.get_info().iter());
    }

    pub fn get_info_add (&self, ret : &mut Vec<f32>, add: Point) { 
	ret.extend(self.a.get_info_add(add).iter());
	ret.extend(self.b.get_info_add(add).iter());
	ret.extend(self.c.get_info_add(add).iter());
    }

    pub fn hit (&self,x: f32,y :f32) -> bool {
	let alpha = self.a.get_degree_to_point(x,y);
	let beta = self.a.get_degree_to_point(self.b.get_x(),self.b.get_y());
	let gamma = self.a.get_degree_to_point(self.c.get_x(),self.c.get_y());
	if alpha as i32 == beta as i32 {return self.hit2(x,y);}
	if alpha as i32 == gamma as i32 {return self.hit2(x,y);}
	if beta > gamma && beta - gamma > 180.0  {return (alpha > beta || alpha < gamma) && self.hit2(x,y);}
	if beta > gamma {return alpha > gamma && beta > alpha && self.hit2(x,y);}
	if gamma > beta && gamma - beta > 180.0 {return (alpha > gamma || alpha < beta) && self.hit2(x,y);}
	return alpha > beta && gamma > alpha && self.hit2(x,y);
    } 

    pub fn hit2 (&self,x: f32,y :f32) -> bool {
	let alpha = self.b.get_degree_to_point(x,y);
	let beta = self.b.get_degree_to_point(self.a.get_x(),self.a.get_y());
	let gamma = self.b.get_degree_to_point(self.c.get_x(), self.c.get_y());
	if alpha as i32 == beta as i32 {return true;}
	if alpha as i32 == gamma as i32 {return true;}
	if beta > gamma && beta - gamma > 180.0  {return alpha > beta || alpha < gamma;}
	if beta > gamma {return alpha > gamma && beta > alpha;}
	if gamma > beta && gamma - beta > 180.0 {return alpha > gamma || alpha < beta;}
	return alpha > beta && gamma > alpha;
    }

    pub fn move_degree_around_point (&mut self,deg: f32,x: f32,y :f32) {
	self.a.move_degree_around_point(deg,x,y);
	self.b.move_degree_around_point(deg,x,y);
	self.c.move_degree_around_point(deg,x,y);
    } 

}

impl Drop for Triangle {
    fn drop(&mut self) {}
}

#[derive(Clone)]
pub struct FormNT {
    triangles_: Vec<Triangle>,
}


impl FormNT {
    pub fn create(triangles_: Vec<Triangle>) -> FormNT {
	return FormNT {triangles_};
    }

    pub fn create_qua_sta(x : f32, y:f32, l:f32, h:f32) -> FormNT {
	let triangles_ = vec![Triangle::create(Point::create_point(x,y,0.0,0.0,0.0,1.0,0.0,0.0),
					       Point::create_point(x+l,y,0.0,0.0,0.0,1.0,1.0,0.0),
					       Point::create_point(x,y-h,0.0,0.0,0.0,1.0,0.0,1.0)),
		  	      Triangle::create(Point::create_point(x+l,y,0.0,0.0,0.0,1.0,1.0,0.0),
					       Point::create_point(x,y-h,0.0,0.0,0.0,1.0,0.0,1.0),
					       Point::create_point(x+l,y-h,0.0,0.0,0.0,1.0,1.0,1.0))];
	return FormNT {triangles_};
    }

    pub fn get_info (& self) -> Vec<f32> {
	let mut vertices : Vec<f32> = vec![];
	for trian in self.triangles_.iter() {trian.get_info(&mut vertices);}
	return vertices;
    }

    pub fn get_info_add (&mut self,add : Point) -> Vec<f32> {
	let mut vertices : Vec<f32> = vec![];
	for trian in self.triangles_.iter() {trian.get_info_add(&mut vertices,add);}
	return vertices;
    }

    pub fn hit (&mut self,x: f32,y :f32) -> bool {
	for trian in self.triangles_.iter() {if trian.hit(x,y) {return true;}}
	return false;
    }
    
    pub fn set_pos (&mut self,x : f32,y : f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].set_x(x);
	    self.triangles_[index].set_y(y);
            index += 1;
        }
    }

    pub fn move_x (&mut self,length : f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].move_x(length);
            index += 1;
        }
    }

    pub fn move_y (&mut self,length : f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].move_y(length);
            index += 1;
        }
    }

    pub fn move_degree_around_point (&mut self,deg: f32,x: f32,y :f32) {
	let mut index : usize = 0;
	loop{
            if index >= self.triangles_.len() {break;}
	    self.triangles_[index].move_degree_around_point(deg,x,y);
            index += 1;
        }
    }

}

impl Drop for FormNT {
    fn drop(&mut self) {}
}

