use crate::extensions::NodeExt;
use gdnative::api::*;
use gdnative::*;
use rand::seq::SliceRandom;

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
#[user_data(user_data::MutexData<Mob>)]
pub struct Mob {
    #[property(default = 150.0)]
    pub min_speed: f32,
    #[property(default = 250.0)]
    pub max_speed: f32,
}

#[derive(Copy, Clone)]
enum MobType {
    Walk,
    Swim,
    Fly,
}

impl MobType {
    fn to_str(self) -> String {
        match self {
            MobType::Walk => "walk".to_string(),
            MobType::Swim => "swim".to_string(),
            MobType::Fly => "fly".to_string(),
        }
    }
}

const MOB_TYPES: [MobType; 3] = [MobType::Walk, MobType::Swim, MobType::Fly];

#[methods]
impl Mob {
    fn _init(_owner: &RigidBody2D) -> Self {
        Mob {
            min_speed: 150.0,
            max_speed: 250.0,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &RigidBody2D) {
        let mut rng = rand::thread_rng();
        let animated_sprite =
            unsafe { owner.get_typed_node::<AnimatedSprite, _>("animated_sprite") };
        animated_sprite.set_animation(MOB_TYPES.choose(&mut rng).unwrap().to_str().into())
    }

    #[export]
    fn on_visibility_screen_exited(&self, owner: &RigidBody2D) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }

    #[export]
    fn on_start_game(&self, owner: &RigidBody2D) {
        unsafe {
            owner.assume_unique().queue_free();
        }
    }
}
