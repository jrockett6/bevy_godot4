pub use godot;

use bevy::prelude::*;
use godot::prelude::*;

struct BevyExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for BevyExtensionLibrary {}

#[derive(GodotClass)]
#[class(base=Node)]
struct BevyApp {
    app: App,
}

#[godot_api]
impl NodeVirtual for BevyApp {
    fn init(_base: Base<Node>) -> Self {
        Self {
            app: App::default(),
        }
    }

    fn ready(&mut self) {
        println!("ready!!!!!!!!");
    }

    fn process(&mut self, _delta: f64) {
        self.app.update();
    }
}
