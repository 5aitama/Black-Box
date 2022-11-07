pub struct Camera {
    /// The camera projection matrix.
    proj_matrix: nalgebra::Matrix4<f32>,

    /// The camera view matrix.
    view_matrix: nalgebra::Matrix4<f32>,

    /// The current camera position.
    position: nalgebra_glm::Vec3,

    /// The current camera scale.
    scale: nalgebra_glm::Vec3,

    /// The current camera rotation.
    rotation: nalgebra_glm::Quat,

    /// The camera width.
    width: f32,

    /// The camera height.
    height: f32,

    /// The camera fov.
    fov: f32,

    /// The camera near.
    near: f32,

    /// The camera far.
    far: f32,
}

impl Camera {
    /// Create a new [Camera].
    /// 
    /// # Arguments
    /// 
    /// * `width`   - The screen width.
    /// * `height`  - The screen height.
    /// * `near`    - The camera near.
    /// * `far`     - The camera far.
    /// * `fov`     - The camera fov **in degrees**
    pub fn new(width: f32, height: f32, near: f32, far: f32, fov: f32) -> Self {

        let proj_matrix = nalgebra_glm::perspective_fov_lh(fov.to_radians(), width, height, near, far);
        let view_matrix = nalgebra_glm::identity();

        Self {
            proj_matrix,
            view_matrix,

            position: nalgebra_glm::Vec3::new(0.0, 0.0, 0.0),
            rotation: nalgebra_glm::quat_identity(),
            scale: nalgebra_glm::Vec3::new(1.0, 1.0, 1.0),

            near,
            far,
            fov,

            width,
            height,
        }
    }

    /// Compute the TRS (Transform Rotation Scale) matrix.
    pub fn get_trs_matrix(&self) -> nalgebra_glm::Mat4x4 {
        let t_matrix = nalgebra_glm::translate(&nalgebra_glm::identity(), &self.position);
        let r_matrix = nalgebra_glm::quat_to_mat4(&self.rotation);
        let s_matrix = nalgebra_glm::scale(&nalgebra_glm::identity(), &self.scale);

        nalgebra_glm::Mat4x4::identity() * s_matrix * r_matrix * t_matrix
    }
    
    /// Compute the projection view matrix.
    pub fn get_proj_view_matrix(&self) -> nalgebra_glm::Mat4x4 {
        self.proj_matrix * self.view_matrix * self.get_trs_matrix()
    }

    /// Get the camera projection width
    pub fn get_width(&self) -> f32 {
        self.width
    }

    /// Get the camera projection height
    pub fn get_height(&self) -> f32 {
        self.height
    }

    /// Get the camera projection fov
    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    /// Get the camera projection near
    pub fn get_near(&self) -> f32 {
        self.near
    }

    /// Get the camera projection far
    pub fn get_far(&self) -> f32 {
        self.far
    }

    /// Translate the camera.
    pub fn translate(&mut self, v: &nalgebra_glm::Vec3) {
        self.position += v;
    }

    pub fn set_position(&mut self, p: &nalgebra_glm::Vec3) {
        self.position = *p;
    }

}