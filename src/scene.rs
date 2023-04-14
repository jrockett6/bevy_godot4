use crate::prelude::*;
use godot::engine::{
    node::InternalMode, packed_scene::GenEditState, resource_loader::CacheMode, ResourceLoader,
};

pub(crate) struct PackedScenePlugin;
impl Plugin for PackedScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_scene.in_base_set(CoreSet::PostUpdate));
    }
}

/// A to-be-instanced-and-spawned Godot scene.
///
/// [`GodotScene`]s that are spawned/inserted into the bevy world will be instanced from the provided
/// handle/path and the instance will be added as an [`ErasedGd`] in the next PostUpdateFlush set.
/// (see [`spawn_scene`])
#[derive(Debug, Component)]
pub enum GodotScene {
    Path(String),
    Handle(Handle<ErasedGdResource>),
}

impl GodotScene {
    pub fn from_path(path: &str) -> Self {
        Self::Path(path.to_string())
    }

    pub fn from_handle(handle: &Handle<ErasedGdResource>) -> Self {
        Self::Handle(handle.clone())
    }
}

#[derive(Component, Debug, Default)]
struct GodotSceneSpawned;

fn spawn_scene(
    mut commands: Commands,
    mut new_scenes: Query<(&mut GodotScene, Entity), Without<GodotSceneSpawned>>,
    mut assets: ResMut<Assets<ErasedGdResource>>,
    mut scene_tree: SceneTreeRef,
) {
    for (scene, ent) in new_scenes.iter_mut() {
        let mut resource_loader = ResourceLoader::singleton();
        let packed_scene = match &scene.as_ref() {
            GodotScene::Handle(handle) => assets
                .get_mut(handle)
                .expect("packed scene to exist in assets")
                .get(),
            GodotScene::Path(path) => resource_loader
                .load(
                    path.into(),
                    "PackedScene".into(),
                    CacheMode::CACHE_MODE_REUSE,
                )
                .expect("packed scene to load"),
        };

        let instance = packed_scene
            .try_cast::<PackedScene>()
            .expect("resource to be a packed scene")
            .instantiate(GenEditState::GEN_EDIT_STATE_DISABLED)
            .unwrap();

        scene_tree.get().get_current_scene().unwrap().add_child(
            instance.share(),
            false,
            InternalMode::INTERNAL_MODE_DISABLED,
        );

        commands
            .entity(ent)
            .insert(ErasedGd::new(instance))
            .insert(GodotSceneSpawned);
    }
}
