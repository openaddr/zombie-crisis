use crate::const_data::Direction;

use crate::player::Player;
use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, NavigationAgent2D};
use godot::prelude::*;
use crate::global::global;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
struct Enemy {
    base: Base<CharacterBody2D>,

    #[init(val = 80.0)]
    speed: f32,

    direction: Direction,

    #[init(node = "AnimatedSprite2D")]
    animator: OnReady<Gd<AnimatedSprite2D>>,

    #[init(node = "Nav/NavigationAgent2D")]
    navi: OnReady<Gd<NavigationAgent2D>>,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn physics_process(&mut self, delta: f64) {
        let gd = global();
        let position = gd.bind().players[0].get_position(); // todo 需要防护
        self.navi.set_target_position(position);
        if self.navi.is_navigation_finished() {
            return;
        }
        let speed = self.speed;
        let cur_position = self.base().get_position();
        let next_position = self.navi.get_next_path_position();
        
        if next_position == cur_position { 
            return;
        }

        let direction = cur_position.direction_to(next_position);

        self.base_mut().set_velocity(direction * speed);
        self.base_mut().move_and_slide();
    }
}
