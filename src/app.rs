use crate::prelude::*;
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
pub struct BevyApp {
    app: Option<App>,
}

impl BevyApp {
    pub fn get_app(&self) -> Option<&App> {
        self.app.as_ref()
    }

    pub fn get_app_mut(&mut self) -> Option<&mut App> {
        self.app.as_mut()
    }
}

#[godot_api]
impl INode for BevyApp {
    fn init(_base: Base<Node>) -> Self {
        Default::default()
    }

    fn ready(&mut self) {
        if godot::engine::Engine::singleton().is_editor_hint() {
            return;
        }

        let mut app = App::new();
        (APP_BUILDER_FN.lock().unwrap().as_mut().unwrap())(&mut app);
        app.add_plugins(bevy::core::TaskPoolPlugin::default())
            .add_plugins(bevy::log::LogPlugin::default())
            .add_plugins(bevy::core::TypeRegistrationPlugin)
            .add_plugins(bevy::core::FrameCountPlugin)
            .add_plugins(bevy::diagnostic::DiagnosticsPlugin)
            .add_plugins(bevy::time::TimePlugin)
            .add_plugins(bevy::hierarchy::HierarchyPlugin)
            .add_plugins(crate::scene::PackedScenePlugin)
            .init_non_send_resource::<crate::scene_tree::SceneTreeRefImpl>();
        // .add_plugins(GodotSignalsPlugin)
        // .add_plugins(GodotInputEventPlugin);

        #[cfg(feature = "assets")]
        app.add_plugins(crate::assets::GodotAssetsPlugin);

        self.app = Some(app);
    }

    fn process(&mut self, _delta: f64) {
        if godot::engine::Engine::singleton().is_editor_hint() {
            return;
        }

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
        if godot::engine::Engine::singleton().is_editor_hint() {
            return;
        }

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
