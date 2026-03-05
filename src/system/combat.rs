use bevy::prelude::*;

use crate::component::{enemy::Enemy, health::Health, player::Player, spell::Spell};

pub fn handle_spell_collisions(
    mut commands: Commands,
    spell_query: Query<(Entity, &Transform, &Spell), (Without<Enemy>, Without<Player>)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    for (spell_entity, spell_transform, spell) in &spell_query {
        for (_enemy_entity, enemy_transform, mut enemy_health) in &mut enemy_query {
            let distance = spell_transform
                .translation
                .distance(enemy_transform.translation);
            // Simple distance check (~30 pixels)
            if distance < 30.0 {
                enemy_health.0 -= spell.damage;
                commands.entity(spell_entity).despawn();
                break;
            }
        }
    }
}

pub fn handle_enemy_player_collisions(
    mut player_query: Query<(&Transform, &mut Health), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok((player_transform, mut player_health)) = player_query.get_single_mut() {
        for enemy_transform in &enemy_query {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < 30.0 {
                player_health.0 -= 1.0; // Subtract some health
            }
        }
    }
}

pub fn handle_death(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in &query {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
