use corange::*;

#[derive(Clone)]
pub enum CameraType {
    Manual,
    Orbit,
    Free,
}

#[derive(Clone)]
pub struct Camera {
    pub position: vec3,
    pub target: vec3,
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub movement: CameraType,
    pub frame: u64
}

impl Default for Camera {
    fn default() -> Camera {
        unsafe {
            Camera {
                movement: CameraType::Free,
                position: vec3_new(10.0, 10.0, 10.0),
                target: vec3_zero(),
                far_clip: 512.0,
                near_clip: 0.10,
                fov:  0.78,
                frame: 0
            }
        }
    }
}

impl Camera {
    pub fn apply(self, camera:*mut camera) {
        unsafe {
            (*camera).position = self.position;
            (*camera).target = self.target;
            (*camera).far_clip = self.far_clip;
            (*camera).near_clip = self.near_clip;
            (*camera).fov = self.fov;
        }
    }
}
