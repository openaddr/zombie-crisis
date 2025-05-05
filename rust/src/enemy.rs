use crate::const_data::Direction;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
struct Enemy {
    base: Base<CharacterBody2D>,

    #[init(val = 80.0)]
    speed: f32,

    direction: Direction,

    #[init(node = "AnimatedSprite2D")]
    animator: OnReady<Gd<AnimatedSprite2D>>,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn physics_process(&mut self, delta: f64) {
        let speed = self.speed;
        self.base_mut().set_velocity(Vector2::new(1.0, 0.0) * speed); // todo
        self.base_mut().move_and_slide();
    }
}
