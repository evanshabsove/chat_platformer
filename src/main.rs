use bevy::{
    prelude::*,
    render::camera::{DepthCalculation, ScalingMode, WindowOrigin},
};

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
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    let projection: OrthographicProjection = OrthographicProjection {
        top: 1.0,
        bottom: -1.0,
        right: 1.0 * RESOLUTION,
        left: -1.0 * RESOLUTION,
        near: 1.0,
        far: -1.0,
        scaling_mode: ScalingMode::None,
        window_origin: WindowOrigin::Center,
        scale: 1.0,
        depth_calculation: DepthCalculation::Distance,
    };

    camera.projection = projection;

    commands.spawn_bundle(camera);
}

fn spawn_player(mut commands: Commands) {}

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
