use bevy::prelude::*;

use crate::component::{
    enemy::Enemy,
    fireball::{Fireball, FireballAnimation},
    orb::Orb,
    player::{Player, PlayerAnimation, PlayerAttackMode},
};

pub fn fire_fireballs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite, &mut PlayerAnimation), With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyX) {
        if let (Ok((player_transform, sprite, mut player_animation)), Ok(enemy_transform)) =
            (player_query.get_single_mut(), enemy_query.get_single())
        {
            if matches!(player_animation.attack_mode, PlayerAttackMode::None) {
                player_animation.attack_mode = PlayerAttackMode::Fireball;
                player_animation.first_frame = 36;
                player_animation.last_frame = 46;

                let texture_handle = asset_server.load("HumansProjectiles.png");
                let texture_atlas =
                    TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                let fb_offset_x = if sprite.flip_x { -20.0 } else { 20.0 };

                // **Step 1:** Calculate direction from player to block
                let direction =
                    (enemy_transform.translation - player_transform.translation).truncate();
                let fireball_direction = direction.normalize_or_zero();

                // **Step 2:** Calculate rotation angle to face the target
                let fireball_rotation =
                    Quat::from_rotation_z(fireball_direction.y.atan2(fireball_direction.x));

                commands.spawn((
                    Sprite {
                        image: texture_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_handle,
                            index: 5,
                        }),
                        ..Default::default()
                    },
                    Transform {
                        translation: player_transform.translation
                            + Vec3 {
                                x: fb_offset_x,
                                y: 0.0,
                                z: 0.0,
                            },
                        rotation: fireball_rotation, // Rotate towards block
                        scale: Vec3::splat(4.0),
                    },
                    Fireball {
                        progress: 0.0,
                        direction: fireball_direction,
                    },
                    FireballAnimation {
                        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                        first_frame: 5,
                        last_frame: 6,
                    },
                ));
            }
        }
    }
}

pub fn fire_orbs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Sprite, &mut PlayerAnimation), With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        if let Ok((player_transform, sprite, mut player_animation)) = player_query.get_single_mut()
        {
            if matches!(player_animation.attack_mode, PlayerAttackMode::None) {
                player_animation.attack_mode = PlayerAttackMode::Orb;
                player_animation.first_frame = 60;
                player_animation.last_frame = 64;

                let texture_handle = asset_server.load("HumansProjectiles.png");
                let texture_atlas =
                    TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 5, None, None);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                let offset_x = if sprite.flip_x { -20.0 } else { 20.0 };

                commands.spawn((
                    Sprite {
                        image: texture_handle.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_handle,
                            index: 10,
                        }),
                        flip_x: sprite.flip_x,
                        ..Default::default()
                    },
                    Transform {
                        translation: player_transform.translation
                            + Vec3 {
                                x: offset_x,
                                y: 0.0,
                                z: 0.0,
                            },
                        scale: Vec3::splat(4.0),
                        ..Default::default()
                    },
                    Orb,
                ));
            }
        }
    }
}
