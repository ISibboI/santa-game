use bevy::prelude::*;
use std::ops::DerefMut;

#[derive(Default)]
pub struct AssetsLoading {
    loaded_count: usize,
    error_count: usize,
    remaining: Vec<HandleUntyped>,
}

#[derive(Component)]
pub struct Assets {
    // Fonts
    font: Handle<Font>,

    // Speech
    speech: HashMap<String, Handle<Audio>>,

    // Textures
    santa: Handle<TextureAtlas>,
    snowflakes: Vec<Handle<TextureAtlas>>,
    outside_background: Handle<TextureAtlas>,
}

fn load_asset<P: AssetPath, R>(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>, path: P) -> Handle<R> {
    let asset = server.load(path);
    loading.remaining.push(asset.clone_untyped());
    asset
}

fn load_speech<P: AssetPath>(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>, mut assets: ResMut<Assets>, path: P) {
    let name = path.to_string();
    let name = name.split("/").last().unwrap().to_owned();

    let speech = load_asset(server, loading, path);
    assets.speech.insert(name, speech);
}

pub fn load_assets_system(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut commands: Commands,
) {
    // Textures
    let santa = load_asset(server, loading, "texture/santa_spritesheet.png");
    let santa = TextureAtlas::from_grid(santa, Vec2::new(32.0, 51.0), 3, 1);

    let snowflake = load_asset(server, loading, "texture/snowflake_spritesheet.png");
    let mut snowflakes = TextureAtlas::new_empty(snowflake);
    snowflakes.add_texture(Rect::new(0.0, 0.0, 13.0, 13.0));
    snowflakes.add_texture(Rect::new(13.0, 0.0, 7.0, 7.0));
    snowflakes.add_texture(Rect::new(20.0, 0.0, 7.0, 7.0));
    snowflakes.add_texture(Rect::new(13.0, 7.0, 4.0, 4.0));

    let outside_background = load_asset(server, loading, "texture/background_outside_spritesheet.png");
    let outside_background = TextureAtlas::from_grid(outside_background, Vec2::new(540.0, 210.0), 1, 1);

    let mut assets = Assets {
        // Fonts
        font: load_asset(server, loading, "font/square.ttf"),

        // Speech
        speech: Default::default(),

        // Textures
        santa,
        snowflakes,
        outside_background
    };

    // Speech
    load_speech(server, loading, assets, "speech/arrive_1.ogg");
    load_speech(server, loading, assets, "speech/enter_house_1.ogg");
    load_speech(server, loading, assets, "speech/hello_1.ogg");
    load_speech(server, loading, assets, "speech/hello_2.ogg");
    load_speech(server, loading, assets, "speech/hello_3.ogg");
    load_speech(server, loading, assets, "speech/tutorial_1.ogg");
    load_speech(server, loading, assets, "speech/tutorial_2.ogg");
    load_speech(server, loading, assets, "speech/tutorial_3.ogg");

    commands.insert_resource(assets);
}

pub fn check_assets_ready_system(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    use bevy::asset::LoadState;

    let AssetsLoading {
        loaded_count,
        error_count,
        remaining,
    } = &mut loading.deref_mut();

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



pub struct SantaAssetPlugin;

impl Plugin for SantaAssetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_assets_system.system())
            .add_system(check_assets_ready_system.system());
    }
}