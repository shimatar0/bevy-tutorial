use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1600.0,
                height: 900.0,
                title: "Bevy Tutorial".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .run();
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.custom_size = Some(Vec2::splat(1.0));
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform { translation: Vec3::new(0.0, 0.0, 900.0), ..Default::default() },
        ..Default::default()
    });
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;

    camera.projection.scaling_mode = ScalingMode::None;
    commands.spawn(camera);
}

#[derive(Resource)]
struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_aliases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Some(Vec2::splat(2.0)),
        None,
    );
    let atlas_handle = texture_aliases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}
