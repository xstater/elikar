use nalgebra_glm as glm;

pub struct Camera2d {
    viewport : (u32,u32,u32,u32)
}

impl Camera2d {
    pub fn projection_matrix(&self) -> glm::TMat4<f32> {
        glm::ortho()
    }
}