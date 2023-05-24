mod app;
#[cfg(feature = "assets")]
mod assets;
mod erased_gd;
mod scene;
mod scene_tree;
mod utils;

pub use bevy;
pub use godot;
pub mod prelude {
    pub use super::erased_gd::{ErasedGd, ErasedGdResource};
    pub use super::scene::GodotScene;
    pub use super::scene_tree::SceneTreeRef;
    pub use super::utils::{
        AsPhysicsSystem, AsVisualSystem, GodotPhysicsFrame, GodotVisualFrame, SystemDeltaTimer,
    };
    pub use bevy::prelude::*;
    pub use bevy_godot4_proc_macros::bevy_app;
    pub use godot::prelude::*;
}
pub use app::{BevyApp, APP_BUILDER_FN};
