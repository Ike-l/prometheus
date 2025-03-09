#[derive(Debug, Clone, Copy)]
pub enum BindGroupFlags {
    Lights = 1,
    Shadow = 1 << 1,
    Camera = 1 << 2,
    Material = 1 << 3,
    Texture = 1 << 4,
}