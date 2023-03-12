use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE,
};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct EncounterSpawner;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("No map file");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3 {
                        x: x as f32 * TILE_SIZE,
                        y: -(y as f32) * TILE_SIZE,
                        z: 100.0,
                    },
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                if char == '~' {
                    commands.entity(tile).insert(EncounterSpawner);
                }
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn(SpriteBundle {
            sprite: default(),
            transform: default(),
            global_transform: default(),
            texture: default(),
            visibility: Visibility::VISIBLE,
            computed_visibility: ComputedVisibility::INVISIBLE,
        })
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}
