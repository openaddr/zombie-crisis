use crate::const_data::Direction;
use crate::global::global;
use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,

    #[init(val = 100.0)]
    speed: f32,

    direction: Direction,

    #[init(node = "AnimatedSprite2D")]
    animator: OnReady<Gd<AnimatedSprite2D>>,

    #[init(node = "Camera2D")]
    camera: OnReady<Gd<Camera2D>>,
}

impl Player {
    /// 根据方向改变动画
    fn change_animation_by_direction(&mut self) {
        self.animator.set_animation(self.direction.as_str());
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        godot_print!("Player ready");
    }
    fn physics_process(&mut self, _: f64) {
        let direction = Input::singleton().get_vector("ui_left", "ui_right", "ui_up", "ui_down");
        let v = direction * self.speed; // 矢量速度
        self.base_mut().set_velocity(v);
        self.base_mut().move_and_slide();

        let direction_state = Direction::from_direction_vector_vector(&direction); // 获取移动方向

        if let Some(direct) = direction_state {
            // 如果有移动方向
            self.animator.play();
            if direct != self.direction {
                // 如果移动方向发生改变
                self.direction = direct; // 设置当前方向
                self.change_animation_by_direction();
            }
        } else {
            self.animator.pause();
        }

        let gd = global();
        let gd = gd.bind();
        self.camera.set_limit(Side::TOP, 0);  // 每帧设置世界边界, todo 太浪费性能, 是否能优化
        self.camera.set_limit(Side::LEFT, 0);
        self.camera.set_limit(Side::BOTTOM, gd.world_size.unwrap().y);
        self.camera.set_limit(Side::RIGHT, gd.world_size.unwrap().x);

        // todo 根据地图大小限制人物移动范围, 而不是通过空气墙
        
    }
}
