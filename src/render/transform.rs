use nalgebra_glm as glm;
use nalgebra_glm::TMat4;

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Transform {
    translation : glm::TVec3<f32>,
    center : glm::TVec3<f32>,
    scale : glm::TVec3<f32>,
    rotation : glm::Qua<f32>
}

impl Transform {
    pub fn new() -> Transform {
        Transform{
            translation: glm::make_vec3(&[0.0,0.0,0.0]),
            center: glm::make_vec3(&[0.0,0.0,0.0]),
            scale: glm::make_vec3(&[1.0,1.0,1.0]),
            rotation: glm::quat_identity()
        }
    }

    pub fn position(&self) -> (f32,f32,f32) {
        unsafe {
            (  *self.translation.get_unchecked(0),
               *self.translation.get_unchecked(1),
               *self.translation.get_unchecked(2))
        }
    }

    pub fn translation_matrix(&self) -> glm::TMat4<f32> {
        glm::translation(&self.translation)
    }

    pub fn move_to(&mut self,x : f32,y : f32,z : f32) {
        self.translation = glm::make_vec3(&[x,y,z]);
    }

    pub fn move_by(&mut self,dx : f32,dy : f32,dz : f32) {
        self.translation += glm::make_vec3(&[dx,dy,dz]);
    }

    pub fn center(&self) -> (f32,f32,f32) {
        unsafe {
            (  *self.center.get_unchecked(0),
               *self.center.get_unchecked(1),
               *self.center.get_unchecked(2))
        }
    }

    pub fn set_center(&mut self,x : f32,y : f32,z : f32) {
        self.center = glm::make_vec3(&[x,y,z]);
    }

    pub fn scale_matrix(&self) -> glm::TMat4<f32> {
        glm::scaling(&self.scale)
    }

    pub fn scale(&self) -> (f32,f32,f32) {
        unsafe {
            (  *self.scale.get_unchecked(0),
               *self.scale.get_unchecked(1),
               *self.scale.get_unchecked(2))
        }
    }

    pub fn scale_to(&mut self,fx : f32,fy : f32,fz : f32) {
        self.scale = glm::make_vec3(&[fx,fy,fz]);
    }

    pub fn flip_x(&mut self) {
        let x = unsafe { self.scale.get_unchecked_mut(0) };
        *x = -*x;
    }

    pub fn flip_y(&mut self) {
        let y = unsafe { self.scale.get_unchecked_mut(1) };
        *y = -*y;
    }

    pub fn flip_z(&mut self) {
        let z = unsafe { self.scale.get_unchecked_mut(2) };
        *z = -*z;
    }

    pub fn rotation_quaternion(&self) -> glm::Qua<f32> {
        self.rotation
    }

    pub fn rotation_matrix(&self) -> glm::TMat4<f32> {
        glm::quat_to_mat4(&self.rotation)
    }

    pub fn yaw(&self) -> f32 {
        glm::quat_yaw(&self.rotation)
    }

    pub fn pitch(&self) -> f32 {
        glm::quat_pitch(&self.rotation)
    }

    pub fn roll(&self) -> f32 {
        glm::quat_roll(&self.rotation)
    }

    pub fn rotate_x_by(&mut self,angle : f32) {
        self.rotate_by((1.0,0.0,0.0),angle)
    }

    pub fn rotate_y_by(&mut self,angle : f32) {
        self.rotate_by((0.0,1.0,0.0),angle)
    }

    pub fn rotate_z_by(&mut self,angle : f32) {
        self.rotate_by((0.0,0.0,1.0),angle)
    }

    pub fn rotate_by(&mut self,axis : (f32,f32,f32),angle : f32) {
        self.rotation = glm::quat_rotate(
            &self.rotation,angle,&glm::make_vec3(&[axis.0,axis.1,axis.2]));
    }

    pub fn rotate_to(&mut self,axis : (f32,f32,f32), angle : f32) {
        self.rotation = glm::quat_rotate(
            &glm::quat_identity(),angle,&glm::make_vec3(&[axis.0,axis.1,axis.2]));
    }

    pub fn model_matrix(&self) -> TMat4<f32> {
        // move center to (0,0,0)
        let center = glm::translation(&-self.center);
        // scale
        let scale = self.scale_matrix() * center;
        // rotate the model
        let rotation = self.rotation_matrix() * scale;
        // move center back
        let center_back = glm::translation(&self.center) * rotation;
        // translation
        let translation = self.translation_matrix() * center_back;
        return translation;
    }

    pub fn as_2d(&mut self) -> View2d<'_> {
        let (x,y,_) = self.position();
        self.move_to(x,y,1.0);
        View2d{
            transform: self
        }
    }
}

pub struct View2d<'a> {
    transform : &'a mut Transform
}

impl<'a> View2d<'a> {
    pub fn position(&self) -> (f32,f32) {
        let pos = self.transform.position();
        (pos.0,pos.1)
    }

    pub fn move_to(&mut self,x : f32, y : f32) {
        self.transform.move_to(x,y,1.0)
    }

    pub fn move_by(&mut self,dx : f32,dy : f32) {
        self.transform.move_by(dx,dy,0.0);
    }

    pub fn scale(&self) -> (f32,f32) {
        let (fx,fy,_) = self.transform.scale();
        (fx,fy)
    }

    pub fn scale_to(&mut self,fx : f32,fy : f32) {
        self.transform.scale_to(fx,fy,1.0);
    }

    pub fn angle(&self) -> f32 {
        self.transform.yaw()
    }

    pub fn rotate_by(&mut self,angle : f32) {
        self.transform.rotate_z_by(angle);
    }

    pub fn rotate_to(&mut self,angle : f32) {
        let angle = angle - self.angle();
        self.rotate_by(angle);
    }

    pub fn flip_vertical(&mut self) {
        self.transform.flip_x();
    }

    pub fn flip_horizontal(&mut self) {
        self.transform.flip_y()
    }
}

#[cfg(test)]
mod tests{
    use crate::render::transform::Transform;

    #[test]
    fn test() {
        let mut trans = Transform::new();
        trans.flip_x();
        assert_eq!(trans.scale(),(-1.0,1.0,1.0));
        trans.rotate_z_by(43_f32.to_radians());
        assert_eq!(trans.pitch().to_degrees(),43_f32);
    }

    #[test]
    fn test2d() {
        let mut trans = Transform::new();
        {
            let mut trans = trans.as_2d();
            trans.move_by(1.0,2.3);
        }
        dbg!(&trans);
    }
}