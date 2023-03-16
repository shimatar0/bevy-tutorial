use std::iter;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii::{spawn_ascii_sprite, spawn_ascii_text, AsciiSheet, AsciiText},
    fadeout::create_fadeout,
    player::Player,
    GameState, TILE_SIZE,
};

#[derive(Component)]
pub struct Enemy;

pub struct CombatPlugin;

pub struct FightEvent {
    target: Entity,
    damege_amount: isize,
}

#[derive(Component, Inspectable)]
pub struct CombatStats {
    pub health: isize,
    pub max_halth: isize,
    pub attack: isize,
    pub defense: isize,
}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FightEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Combat)
                    .with_system(test_exit_combat)
                    .with_system(combat_input)
                    .with_system(damage_calculation)
                    .with_system(combat_camera),
            )
            .add_system_set(SystemSet::on_enter(GameState::Combat).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(despawn_enemy));
    }
}

fn damage_calculation(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    mut fight_event: EventReader<FightEvent>,
    text_query: Query<&AsciiText>,
    mut target_query: Query<(&Children, &mut CombatStats)>,
) {
    for event in fight_event.iter() {
        let (target_children, mut target_stats) = target_query
            .get_mut(event.target)
            .expect("Fighting target without stats!");
        target_stats.health = std::cmp::max(
            target_stats.health - (event.damege_amount - target_stats.defense),
            0,
        );

        for child in target_children.iter() {
            if text_query.get(*child).is_ok() {
                commands.entity(*child).despawn_recursive();
                let new_health = spawn_ascii_text(
                    &mut commands,
                    &ascii,
                    &format!("Health: {}", target_stats.health),
                    Vec3 {
                        x: -4.5 * TILE_SIZE,
                        y: 2.0 * TILE_SIZE,
                        z: 100.0,
                    },
                );

                commands.entity(event.target).add_child(new_health);
            }
        }

        if target_stats.health == 0 {
            create_fadeout(&mut commands, GameState::Overworld, &ascii)
        }
    }
}

fn combat_input(
    keyboard: Res<Input<KeyCode>>,
    mut fight_event: EventWriter<FightEvent>,
    player_query: Query<&CombatStats, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    let player_stats = player_query.single();
    let target = enemy_query.iter().next().unwrap();
    if keyboard.just_pressed(KeyCode::Return) {
        fight_event.send(FightEvent {
            target: target,
            damege_amount: player_stats.attack,
        });
    }
}

fn combat_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let enemt_health = 3;
    let heath_text = spawn_ascii_text(
        &mut commands,
        &ascii,
        &format!("Health: {}", enemt_health),
        Vec3 {
            x: -4.5 * TILE_SIZE,
            y: 2.0 * TILE_SIZE,
            z: 100.0,
        },
    );

    let sprite = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        'b' as usize,
        Color::rgb(0.8, 0.8, 0.8),
        Vec3::new(0.0, 0.0, 100.0),
        Vec3::splat(1.0),
    );

    commands
        .entity(sprite)
        .insert(Enemy)
        .insert(CombatStats {
            health: enemt_health,
            max_halth: enemt_health,
            attack: 2,
            defense: 1,
        })
        .insert(Name::new("Bat"))
        .add_child(heath_text);
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn test_exit_combat(mut keyboard: ResMut<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Changing to Overworld");
        state.set(GameState::Overworld).unwrap();
        keyboard.clear();
    }
}
