use zuri_net_derive::packet;

use crate::proto::ints::{VarU32, VarU64};

/// Sent by the server to animate an entity client-side. It may be used to play a single animation,
/// or to activate a controller which can start a sequence of animations based on different
/// conditions specified in an animation controller.
/// https://minecraft.gamepedia.com/Bedrock_Edition_beta_animation_documentation
#[packet]
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
    #[size_type(VarU32)]
    pub entity_runtime_ids: Vec<VarU64>,
}
