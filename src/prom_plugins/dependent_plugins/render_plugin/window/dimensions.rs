use small_read_only::ReadOnly;
use winit::dpi::PhysicalSize;

#[derive(Debug, Clone, Copy, ReadOnly)]
pub struct WindowDimensions {
    width: u32,
    height: u32,
}

impl WindowDimensions {
    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

impl From<PhysicalSize<u32>> for WindowDimensions {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self {
            width: value.width,
            height: value.height
        }
    }
}