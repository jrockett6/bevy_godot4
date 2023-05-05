use super::prelude::{ErasedInputEvent, GodotPhysicsFrame, GodotVisualFrame};
use bevy::prelude::*;
use godot::{engine::InputEvent, prelude::*};
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
    /// Get access to the Bevy app.
    ///
    /// This is useful if you need to modify the Bevy [World] directly (via `App.world`)
    ///
    /// SAFETY: Make sure this is not called before the app has been added to the engine - ready(
    /// must be called by Godot prior to accessing the app
    pub fn get_app(&mut self) -> &mut App {
        self.app
            .as_mut()
            .expect("ready() to be called by Godot prior to this")
    }
}

#[godot_api]
impl NodeVirtual for BevyApp {
    fn init(_base: Base<Node>) -> Self {
        Default::default()
    }

    fn ready(&mut self) {
        // Build Bevy app
        let mut app = App::new();
        app
            // Necessary Bevy plugins
            .add_plugin(bevy::core::TaskPoolPlugin::default())
            .add_plugin(bevy::log::LogPlugin::default())
            .add_plugin(bevy::core::TypeRegistrationPlugin)
            .add_plugin(bevy::core::FrameCountPlugin)
            .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
            .add_plugin(bevy::time::TimePlugin)
            .add_plugin(bevy::hierarchy::HierarchyPlugin)
            // Crate plugins, resources, and events
            .add_plugin(crate::scene::PackedScenePlugin)
            .add_plugin(crate::assets::GodotAssetsPlugin)
            .init_non_send_resource::<crate::scene_tree::SceneTreeRefImpl>()
            .add_event::<ErasedInputEvent>();

        // Build user provided app
        (APP_BUILDER_FN.lock().unwrap().as_mut().unwrap())(&mut app);

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

    fn input(&mut self, event: Gd<InputEvent>) {
        // let event: Gd<InputEvent> = Gd::try_from_instance_id(event.instance_id()).unwrap();
        // println!("{:?}", event);
        // println!("{:?}", input_event.get());
        let input_event = ErasedInputEvent::new(event);
        self.app
            .as_mut()
            .unwrap()
            .world
            .get_resource_mut::<Events<ErasedInputEvent>>()
            .unwrap()
            .send(input_event);
    }
}
