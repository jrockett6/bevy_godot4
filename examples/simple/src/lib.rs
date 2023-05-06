use bevy::ecs::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_godot4::prelude::*;
use godot::engine::Sprite2D;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct MyAssets {
    #[asset(path = "sprite.tscn")]
    pub sprite: Handle<ErasedGdResource>,
}

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::Loading)
        .add_system(spawn_sprite.in_schedule(OnEnter(GameState::Playing)))
        .add_system(
            move_sprite
                .as_physics_system()
                .run_if(in_state(GameState::Playing)),
        );
    // .add_system(hello_physics_update.as_physics_system())
    // .add_system(hello_visual_update.as_visual_system());
}

fn spawn_sprite(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn(
        GodotScene::from_handle(&assets.sprite).with_translation2d(Vector2 { x: 200.0, y: 200.0 }),
    );
}

fn move_sprite(mut sprite: Query<&mut ErasedGd>, mut delta: SystemDeltaTimer) {
    if let Ok(mut sprite) = sprite.get_single_mut() {
        let mut sprite = sprite.get::<Sprite2D>();
        let delta = delta.delta_seconds() * 20.0;
        let position = sprite.get_position();

        sprite.set_position(Vector2 {
            x: position.x + delta,
            y: position.y + delta,
        });
    }
}

#[allow(dead_code)]
fn hello_physics_update() {
    println!("hello physics update")
}

#[allow(dead_code)]
fn hello_visual_update() {
    println!("hello visual update")
}
