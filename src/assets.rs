use bevy::prelude::*;
use std::ops::DerefMut;

#[derive(Default)]
pub struct AssetsLoading {
    loaded_count: usize,
    error_count: usize,
    remaining: Vec<HandleUntyped>,
}

pub struct Assets {
    santa: Handle<TextureAtlas>,
    snowflake: Handle<TextureAtlas>,
}

pub fn load_assets_system(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut assets: ResMut<Assets>,
) {
    let santa = server.load("texture/santa_spritesheet.png");
    let snowflake = server.load("texture/snowflake_spritesheet.png");

    loading.remaining.push(santa.clone_untyped());
    loading.remaining.push(snowflake.clone_untyped());

    *(assets.deref_mut()) = Assets { santa, snowflake };
}

pub fn check_assets_ready_system(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    use bevy::asset::LoadState;

    let AssetsLoading {
        loaded_count,
        error_count,
        remaining,
    } = loading.deref_mut();

    remaining.retain(|handle| match server.get_load_state(handle) {
        LoadState::Failed => {
            error!("Could not load asset {:?}", server.get_handle_path(handle));
            *error_count += 1;
            false
        }
        LoadState::Loaded => {
            *loaded_count += 1;
            true
        }
        _ => false,
    });

    if remaining.is_empty() {
        if *error_count == 0 {
            info!("Loaded all assets successfully");
        } else {
            info!(
                "Finished loading {} assets with {} errors",
                *loaded_count + *error_count,
                error_count
            )
        }
    }
}
