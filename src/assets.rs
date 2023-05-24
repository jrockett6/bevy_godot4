use crate::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    utils::BoxedFuture,
};
use godot::engine::{resource_loader::CacheMode, ResourceLoader};

pub struct GodotAssetsPlugin;
impl Plugin for GodotAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AssetPlugin {
            asset_folder: std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            watch_for_changes: false,
        })
        .add_asset::<ErasedGdResource>()
        .init_asset_loader::<GodotResourceLoader>();
    }
}

/// Allow for loading godot resources via Bevy's assets framework, can be used with bevy_asset_loader
///
/// This is not a recommended feature due to issues with referencing a PackedScene resource 
/// simultaneously in Godot during loading - and there currently isn't an easy way to make asset 
/// loading into a NonSend Bevy Resource single-threaded.
#[derive(Default)]
pub struct GodotResourceLoader;

impl AssetLoader for GodotResourceLoader {
    fn extensions(&self) -> &[&str] {
        &["tscn", "scn", "res", "tres", "jpg", "png"]
    }

    fn load<'a>(
        &'a self,
        _bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async {
            let mut load_asset = || {
                let mut resource_loader = ResourceLoader::singleton();
                let loaded = resource_loader
                    .load(
                        ("res://".to_owned()
                            + load_context.path().to_str().ok_or_else(|| {
                                anyhow::anyhow!("failed to convert asset path to string")
                            })?)
                        .into(),
                        "".into(),
                        CacheMode::CACHE_MODE_REUSE,
                    )
                    .ok_or_else(|| {
                        anyhow::anyhow!("failed to load asset {}", load_context.path().display())
                    })?;

                load_context.set_default_asset(LoadedAsset::new(ErasedGdResource::new(loaded)));
                Ok(())
            };

            if let Err(e) = load_asset() {
                eprintln!(
                    "loading {} asset failed: {}",
                    load_context.path().to_str().unwrap(),
                    e
                );
                return Err(e);
            }

            Ok(())
        })
    }
}
