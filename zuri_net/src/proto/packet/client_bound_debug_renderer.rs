use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ClientBoundDebugRendererType {
    None,
    Clear,
    AddCube,
}

/// Sent by the server to spawn an outlined cube on client-side.
#[derive(Debug, Clone)]
pub struct ClientBoundDebugRenderer {
    /// The type of action to perform on the renderer, usually to add or clear a cube.
    pub render_type: ClientBoundDebugRendererType,
    /// The text that is displayed above the debug.
    pub text: String,
    /// The position to spawn the debug on.
    pub position: Vec3,
    /// The red value from the RGBA colour rendered on the debug. This value is in the range 0-1.
    pub red: f32,
    /// The green value from the RGBA colour rendered on the debug. This value is in the range 0-1.
    pub green: f32,
    /// The blue value from the RGBA colour rendered on the debug. This value is in the range 0-1.
    pub blue: f32,
    /// The alpha value from the RGBA colour rendered on the debug. This value is in the range 0-1.
    pub alpha: f32,
    /// The duration the debug will last in the world for. It is measured in milliseconds.
    pub duration: i64,
}

impl PacketType for ClientBoundDebugRenderer {
    fn write(&self, writer: &mut Writer) {
        writer.i32(self.render_type.to_i32().unwrap());
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
        let render_type = ClientBoundDebugRendererType::from_i32(reader.i32()).unwrap();
        Self {
            render_type,
            text: if render_type == ClientBoundDebugRendererType::AddCube { reader.string() } else { String::new() },
            position: if render_type == ClientBoundDebugRendererType::AddCube { reader.vec3() } else { Vec3::new(0.0, 0.0, 0.0) },
            red: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            green: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            blue: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            alpha: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            duration: if render_type == ClientBoundDebugRendererType::AddCube { reader.i64() } else { 0 },
        }
    }
}
