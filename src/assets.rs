use bevy::asset::{Asset, AssetPath};
use bevy::prelude::*;
use bevy::sprite::Rect;
use std::collections::HashMap;
use std::ops::DerefMut;

#[derive(Default)]
pub struct AssetsLoading {
    loaded_count: usize,
    error_count: usize,
    remaining: Vec<HandleUntyped>,
}

pub struct SantaAssets {
    // Fonts
    pub font: Handle<Font>,

    // Speech
    pub speech: HashMap<String, Handle<AudioSource>>,

    // Textures
    pub santa: Handle<TextureAtlas>,
    pub snowflakes: Handle<TextureAtlas>,
    pub outside_background: Handle<TextureAtlas>,
}

fn load_asset<'a, P: Into<AssetPath<'a>>, R: Asset>(
    server: &Res<AssetServer>,
    loading: &mut ResMut<AssetsLoading>,
    path: P,
) -> Handle<R> {
    let asset = server.load(path);
    loading.remaining.push(asset.clone_untyped());
    asset
}

fn load_speech<'a, P: Into<AssetPath<'a>> + ToString>(
    server: &Res<AssetServer>,
    loading: &mut ResMut<AssetsLoading>,
    assets: &mut SantaAssets,
    path: P,
) {
    let name = path.to_string();
    let name = name.split("/").last().unwrap().to_owned();

    let speech = load_asset(server, loading, path);
    assets.speech.insert(name, speech);
}

fn load_assets_system(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    // Textures
    let santa = load_asset(&server, &mut loading, "texture/santa_spritesheet.png");
    let mut santa = TextureAtlas::new_empty(santa, Vec2::new(111.0, 51.0));
    santa.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(36.0, 51.0),
    });
    santa.add_texture(Rect {
        min: Vec2::new(36.0, 0.0),
        max: Vec2::new(75.0, 51.0),
    });
    santa.add_texture(Rect {
        min: Vec2::new(75.0, 0.0),
        max: Vec2::new(111.0, 51.0),
    });
    let santa = texture_atlases.add(santa);

    let snowflake = load_asset(&server, &mut loading, "texture/snowflake_spritesheet.png");
    let mut snowflakes = TextureAtlas::new_empty(snowflake, Vec2::new(27.0, 13.0));
    snowflakes.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(13.0, 13.0),
    });
    snowflakes.add_texture(Rect {
        min: Vec2::new(13.0, 0.0),
        max: Vec2::new(20.0, 7.0),
    });
    snowflakes.add_texture(Rect {
        min: Vec2::new(20.0, 0.0),
        max: Vec2::new(27.0, 7.0),
    });
    snowflakes.add_texture(Rect {
        min: Vec2::new(13.0, 7.0),
        max: Vec2::new(17.0, 11.0),
    });
    let snowflakes = texture_atlases.add(snowflakes);

    let outside_background = load_asset(
        &server,
        &mut loading,
        "texture/background_outside_spritesheet.png",
    );
    let outside_background =
        TextureAtlas::from_grid(outside_background, Vec2::new(540.0, 210.0), 1, 1);
    let outside_background = texture_atlases.add(outside_background);

    let mut assets = SantaAssets {
        // Fonts
        font: load_asset(&server, &mut loading, "font/square.ttf"),

        // Speech
        speech: Default::default(),

        // Textures
        santa,
        snowflakes,
        outside_background,
    };

    // Speech
    load_speech(&server, &mut loading, &mut assets, "speech/arrive_1.ogg");
    load_speech(
        &server,
        &mut loading,
        &mut assets,
        "speech/enter_house_1.ogg",
    );
    load_speech(&server, &mut loading, &mut assets, "speech/hello_1.ogg");
    load_speech(&server, &mut loading, &mut assets, "speech/hello_2.ogg");
    load_speech(&server, &mut loading, &mut assets, "speech/hello_3.ogg");
    load_speech(&server, &mut loading, &mut assets, "speech/tutorial_1.ogg");
    load_speech(&server, &mut loading, &mut assets, "speech/tutorial_2.ogg");
    load_speech(&server, &mut loading, &mut assets, "speech/tutorial_3.ogg");

    commands.insert_resource(assets);
}

fn check_assets_ready_system(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    use bevy::asset::LoadState;

    let AssetsLoading {
        loaded_count,
        error_count,
        remaining,
    } = &mut loading.deref_mut();

    let mut has_changed = false;
    remaining.retain(|handle| match server.get_load_state(handle) {
        LoadState::Failed => {
            error!("Could not load asset {:?}", server.get_handle_path(handle));
            *error_count += 1;
            has_changed = true;
            false
        }
        LoadState::Loaded => {
            *loaded_count += 1;
            has_changed = true;
            false
        }
        _ => true,
    });

    if remaining.is_empty() && has_changed {
        if *error_count == 0 {
            info!("Loaded all {} assets successfully", loaded_count);
        } else {
            info!(
                "Finished loading {} assets with {} errors",
                *loaded_count + *error_count,
                error_count
            )
        }
    }
}

pub struct SantaAssetPlugin;

impl Plugin for SantaAssetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AssetsLoading>()
            .add_startup_system_to_stage(StartupStage::PreStartup, load_assets_system.system())
            .add_system(check_assets_ready_system.system());
    }
}
