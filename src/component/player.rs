use bevy::{math::Vec2, prelude::Component, sprite::Sprite, time::Timer};

#[derive(Component, Default)]
#[require(Sprite)]
pub struct Player {
    pub facing_direction: Vec2,
}

pub enum PlayerAttackMode {
    Orb,
    Fireball,
    None,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub timer: Timer,
    pub last_frame: usize,
    pub first_frame: usize,
    pub attack_mode: PlayerAttackMode,
}
