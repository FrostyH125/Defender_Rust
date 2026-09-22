use raylib::math::Vector3;

pub struct Light {
    pub position: Vector3,
    pub color: Vector3,
    pub intensity: f32,
    pub radius: f32
}