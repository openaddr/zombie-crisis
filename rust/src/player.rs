use godot::classes::*;
use godot::prelude::*;
#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
struct Player{
    base: Base<CharacterBody2D>,

    #[init(val = 100.0)]
    speed: f32,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn physics_process(&mut self, _: f64) {
        let direction = Input::singleton().get_vector("ui_left", "ui_right", "ui_up", "ui_down");
        let v = direction * self.speed; // 矢量速度
        self.base_mut().set_velocity(v);
        self.base_mut().move_and_slide();
    }
}
