use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to animate an entity client-side. It may be used to play a single animation,
/// or to activate a controller which can start a sequence of animations based on different
/// conditions specified in an animation controller.
/// https://minecraft.gamepedia.com/Bedrock_Edition_beta_animation_documentation
#[derive(Debug, Clone)]
pub struct AnimateEntity {
    /// The name of a single animation to start playing.
    pub animation: String,
    /// The first state to start with. These states are declared in animation controllers (which, in
    /// themselves, are animations too). These states in turn may have animations and transitions to
    /// move to a next state.
    pub next_state: String,
    /// MoLang expression that specifies when the animation should be stopped.
    pub stop_condition: String,
    /// MoLang stop condition version.
    pub stop_condition_version: i32,
    /// The animation controller that is used to manage animations. These controllers decide when
    /// to play which animation.
    pub controller: String,
    /// It is not clear what the purpose of this field is.
    pub blend_out_time: f32,
    /// List of runtime IDs of entities that the animation should be applied to.
    pub entity_runtime_ids: Vec<u64>,
}

impl PacketType for AnimateEntity {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.animation.as_str());
        writer.string(self.next_state.as_str());
        writer.string(self.stop_condition.as_str());
        writer.i32(self.stop_condition_version);
        writer.string(self.controller.as_str());
        writer.f32(self.blend_out_time);
        writer.var_u32(self.entity_runtime_ids.len() as u32);
        self.entity_runtime_ids.iter().for_each(|runtime_id| writer.var_u64(*runtime_id));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            animation: reader.string(),
            next_state: reader.string(),
            stop_condition: reader.string(),
            stop_condition_version: reader.i32(),
            controller: reader.string(),
            blend_out_time: reader.f32(),
            entity_runtime_ids: (0..reader.var_u32()).map(|_| reader.var_u64()).collect(),
        }
    }
}
