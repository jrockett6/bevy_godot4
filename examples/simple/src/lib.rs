use bevy::{
    app::{App, Update},
    ecs::{schedule::IntoScheduleConfigs, system::Res},
    prelude::{AppExtStates, Commands, OnEnter, Query, Resource, States, in_state},
    state::app::StatesPlugin,
};
use bevy_godot4::prelude::{
    AsPhysicsSystem, ErasedGd, ErasedGdResource, GodotScene, SystemDeltaTimer, bevy_app,
};
use godot::{
    builtin::Vector2,
    classes::{ResourceLoader, Sprite2D},
};
use godot::{init::ExtensionLibrary, prelude::gdextension};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Playing,
}

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .init_resource::<MyAssets>()
        .add_systems(OnEnter(GameState::Playing), spawn_sprite)
        .add_systems(
            Update,
            move_sprite
                .as_physics_system()
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Resource, Debug)]
pub struct MyAssets {
    pub sprite: ErasedGdResource,
}

impl Default for MyAssets {
    fn default() -> Self {
        let mut resource_loader = ResourceLoader::singleton();
        let sprite = ErasedGdResource::new(resource_loader.load("sprite.tscn").unwrap());

        Self { sprite }
    }
}

fn spawn_sprite(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn(
        GodotScene::from_resource(assets.sprite.clone())
            .with_translation2d(Vector2 { x: 200.0, y: 200.0 }),
    );
}

fn move_sprite(mut sprite: Query<&mut ErasedGd>, mut delta: SystemDeltaTimer) {
    if let Ok(mut sprite) = sprite.single_mut() {
        let mut sprite = sprite.get::<Sprite2D>();
        let delta = delta.delta_seconds() * 20.0;
        let position = sprite.get_position();

        sprite.set_position(Vector2 {
            x: position.x + delta,
            y: position.y + delta,
        });
    }
}
