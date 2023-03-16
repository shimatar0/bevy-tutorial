use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct AsciiPlugin;

#[derive(Component)]
pub struct AsciiText;
#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}

pub fn spawn_ascii_text(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    to_print: &str,
    left_center: Vec3,
) -> Entity {
    let color = Color::rgb(0.8, 0.8, 0.8);
    let mut character_sprites = Vec::new();

    for (i, char) in to_print.chars().enumerate() {
        assert!(char as usize <= 255);
        character_sprites.push(spawn_ascii_sprite(
            commands,
            ascii,
            char as usize,
            color,
            Vec3 {
                x: i as f32 * TILE_SIZE,
                y: 0.0,
                z: 0.0,
            },
            Vec3::splat(1.0),
        ))
    }

    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new(format!("Text - {}", to_print)))
        .insert(Transform {
            translation: left_center,
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .insert(AsciiText)
        .push_children(&character_sprites)
        .id()
}

pub fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    assert!(index < 256, "Index out of Ascii Range");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: translation,
                scale: scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

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
