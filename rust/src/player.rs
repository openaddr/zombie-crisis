use crate::const_data::Direction;
use crate::global::Global;
use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,

    #[init(val = 100.0)]
    speed: f32,

    direction: Direction,

    #[init(node = "AnimatedSprite2D")]
    animator: OnReady<Gd<AnimatedSprite2D>>,
}

impl Player {
    /// 根据方向改变动画
    fn change_animation_by_direction(&mut self) {
        self.animator.set_animation(self.direction.as_str());
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn physics_process(&mut self, _: f64) {
        let direction = Input::singleton().get_vector("ui_left", "ui_right", "ui_up", "ui_down");
        let v = direction * self.speed; // 矢量速度
        self.base_mut().set_velocity(v);
        self.base_mut().move_and_slide();

        let mut gd = Engine::singleton()  // todo 单例
            .get_singleton("Global")
            .unwrap()
            .cast::<Global>();
        
        let i = gd.bind().int;
        gd.bind_mut().int = i + 1;
        godot_print!("{:?}", gd.bind().int);

        let direction_state = Direction::from_direction_vector_vector(&direction); // 获取移动方向

        if let Some(direct) = direction_state {
            // 如果有移动方向
            if direct != self.direction {
                // 如果移动方向发生改变
                self.direction = direct; // 设置当前方向
                self.change_animation_by_direction();
            }
        }
    }
}
