use raylib::math::Vector2;
use zander_game_core_rs::raylib::{
    animation_data::SpriteAnimationData, sprite::Sprite, sprite_animation::SpriteAnimationInstance,
};

use crate::entities::{
    character::{
        Affiliation, Character, CharacterData, CharacterKind, CharacterSpecificData,
        SimpleCharacterKind,
    },
    characters::enemy::{Enemy, EnemyData, EnemyKind},
};

static SLIME_IDLE_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &Sprite::create_sequence_of_sprites::<2>(16, 136, 8, 8),
    frame_duration: 0.5,
    should_loop: true,
};

static SLIME_MOVE_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &Sprite::create_sequence_of_sprites::<2>(16, 144, 8, 8),
    frame_duration: 0.25,
    should_loop: true,
};

static SLIME_ATTACK_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &Sprite::create_sequence_of_sprites::<5>(16, 152, 8, 8),
    frame_duration: 0.25,
    should_loop: false,
};

static SLIME_POST_ATTACK_ANIM: SpriteAnimationData = SpriteAnimationData {
    frames: &Sprite::create_sequence_of_sprites::<7>(16, 160, 8, 8),
    frame_duration: 0.25,
    should_loop: false,
};

#[derive(PartialEq)]
pub enum SlimeState {
    Idle,
}

pub struct Slime {
    pub slime_state: SlimeState,
}

impl Slime {
    pub fn new(pos: Vector2) -> Character {
        let character_values = CharacterSpecificData {
            idle_anim: SpriteAnimationInstance::new(&SLIME_IDLE_ANIM),
            move_anim: SpriteAnimationInstance::new(&SLIME_MOVE_ANIM),
            attack_anim: SpriteAnimationInstance::new(&SLIME_ATTACK_ANIM),
            post_attack_anim: SpriteAnimationInstance::new(&SLIME_POST_ATTACK_ANIM),
            draw_offset: Vector2::zero(),
            max_health: 30.0,
            time_between_attacks: 3.0,
            attack_power: 3.0,
            move_speed: 5.0,
            width: 8.0,
            height: 8.0,
            affiliation: Affiliation::Evil,
            character_kind: SimpleCharacterKind::Enemy,
        };

        let data = CharacterData::new(pos, character_values);

        let slime = Slime {
            slime_state: SlimeState::Idle,
        };

        return Character {
            character_data: data,
            character_kind: CharacterKind::Enemy(Enemy {
                enemy_data: EnemyData {},
                enemy_kind: EnemyKind::Slime(slime),
            }),
        };
    }

    pub fn update(&mut self) {
        // Idle: 
        //  find closest target -> MovingToTarget
        // MovingToTarget: 
        //  check closest target every second, target could have died or moved further than another one
        //  once reached a target -> Attacking
        // Attacking:
        //  initiate a battle with target
        //  once done -> Idle
    }

    pub fn current_sprite(&self, character_data: &CharacterData) -> Sprite {
        match self.slime_state {
            SlimeState::Idle => character_data.character_values.idle_anim.current_sprite(),
        }
    }
}
