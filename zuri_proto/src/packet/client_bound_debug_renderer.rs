use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct ClientBoundDebugRenderer {
    pub render_type: ClientBoundDebugRendererType,
    pub text: String,
    pub position: Vec3,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub duration: i64,
}

impl Packet for ClientBoundDebugRenderer {
    fn write(&self, writer: &mut Writer) {
        writer.i32(num::ToPrimitive::to_i32(&self.render_type).unwrap());
        if self.render_type == ClientBoundDebugRendererType::AddCube {
            writer.string(self.text.as_str());
            writer.vec3(self.position);
            writer.f32(self.red);
            writer.f32(self.green);
            writer.f32(self.blue);
            writer.f32(self.alpha);
            writer.i64(self.duration);
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let render_type = num::FromPrimitive::from_i32(reader.i32()).unwrap();
        Self {
            render_type,
            text: if render_type == ClientBoundDebugRendererType::AddCube { reader.string() } else { "".to_string() },
            position: if render_type == ClientBoundDebugRendererType::AddCube { reader.vec3() } else { Vec3::new(0.0, 0.0, 0.0) },
            red: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            green: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            blue: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            alpha: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            duration: if render_type == ClientBoundDebugRendererType::AddCube { reader.i64() } else { 0 },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ClientBoundDebugRendererType {
    None,
    Clear,
    AddCube,
}
