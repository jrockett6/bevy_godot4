pub mod app;
pub mod component;
pub mod utils;

pub use bevy;
pub use godot;
pub mod prelude {
    pub use godot::prelude::gdextension;
    pub use super::component::GdComponent;
    pub use super::utils::{GodotPhysicsFrame, GodotVisualFrame, SystemDeltaTimer};
    pub use bevy_godot4_proc_macros::bevy_app;
    pub use lazy_static;
}
