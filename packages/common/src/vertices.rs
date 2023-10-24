pub mod vertex_data;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex2DColored {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

unsafe impl Zeroable for Vertex2DColored {}
unsafe impl Pod for Vertex2DColored {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex3D {
    pub position: [f32; 3],
}

unsafe impl Zeroable for Vertex3D {}
unsafe impl Pod for Vertex3D {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex4DColored {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

unsafe impl Zeroable for Vertex4DColored {}
unsafe impl Pod for Vertex4DColored {}