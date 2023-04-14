use super::prelude::{GodotPhysicsFrame, GodotVisualFrame};
use bevy::prelude::*;
use godot::prelude::*;
use std::{
    panic::{catch_unwind, resume_unwind, AssertUnwindSafe},
    sync::Mutex,
};

lazy_static::lazy_static! {
    #[doc(hidden)]
    pub static ref APP_BUILDER_FN: Mutex<Option<Box<dyn Fn(&mut App) + Send>>> = Mutex::new(None);
}

#[derive(GodotClass, Default)]
#[class(base=Node)]
struct BevyApp {
    app: Option<App>,
}

#[godot_api]
impl NodeVirtual for BevyApp {
    fn init(_base: Base<Node>) -> Self {
        Default::default()
    }

    fn ready(&mut self) {
        let mut app = App::new();
        (APP_BUILDER_FN.lock().unwrap().as_mut().unwrap())(&mut app);
        app.add_plugin(bevy::core::TaskPoolPlugin::default())
            .add_plugin(bevy::log::LogPlugin::default())
            .add_plugin(bevy::core::TypeRegistrationPlugin)
            .add_plugin(bevy::core::FrameCountPlugin)
            .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
            .add_plugin(bevy::time::TimePlugin)
            .add_plugin(bevy::hierarchy::HierarchyPlugin)
            .add_plugin(crate::scene::PackedScenePlugin)
            .add_plugin(crate::assets::GodotAssetsPlugin)
            .init_non_send_resource::<crate::scene_tree::SceneTreeRefImpl>();
        // .add_plugin(GodotSignalsPlugin)
        // .add_plugin(GodotInputEventPlugin);

        self.app = Some(app);
    }

    fn process(&mut self, _delta: f64) {
        if let Some(app) = self.app.as_mut() {
            app.insert_resource(GodotVisualFrame);

            if let Err(e) = catch_unwind(AssertUnwindSafe(|| app.update())) {
                self.app = None;

                eprintln!("bevy app update panicked");
                resume_unwind(e);
            }

            app.world.remove_resource::<GodotVisualFrame>();
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if let Some(app) = self.app.as_mut() {
            app.insert_resource(GodotPhysicsFrame);

            if let Err(e) = catch_unwind(AssertUnwindSafe(|| app.update())) {
                self.app = None;

                eprintln!("bevy app update panicked");
                resume_unwind(e);
            }

            app.world.remove_resource::<GodotPhysicsFrame>();
        }
    }
}
