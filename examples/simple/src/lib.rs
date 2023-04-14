use bevy::ecs::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_godot4::prelude::*;

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
        .add_system(hello_physics_update.as_physics_system())
        .add_system(hello_visual_update.as_visual_system());
}

fn spawn_sprite(mut commands: Commands, assets: Res<MyAssets>, _scene_tree: SceneTreeRef) {
    commands.spawn(GodotScene::from_handle(&assets.sprite));
}

fn hello_physics_update() {
    println!("hello physics update")
}

fn hello_visual_update() {
    println!("hello visual update")
}
