use bevy_godot4::bevy::prelude::*;
use bevy_godot4::prelude::*;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_system(hello_world_system);
}

fn hello_world_system() {
    println!("hello world")
}
