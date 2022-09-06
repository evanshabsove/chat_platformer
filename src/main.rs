use bevy::{prelude::*, render::texture::ImageSettings};

mod player;
use player::PlayerPugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
fn main() {
    let height: f32 = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Chat Platformer".to_string(),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();

    commands.spawn_bundle(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0),
        Vec2::splat(0.0),
    );

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}
