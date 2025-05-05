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
        let gd = global();
        let gd = gd.bind();
        let world_size = gd.world_size.expect("地图大小值尚未初始化");
        self.camera.set_limit(Side::TOP, 0);  // 每帧设置世界边界, todo 太浪费性能, 是否能优化
        self.camera.set_limit(Side::LEFT, 0);
        self.camera.set_limit(Side::BOTTOM, world_size.y);
        self.camera.set_limit(Side::RIGHT, world_size.x);
        
        godot_print!("world_size {:?}", world_size);
        
        let mut direction = Input::singleton().get_vector("ui_left", "ui_right", "ui_up", "ui_down");
        
        // 如果当前位置在地图边界外, 则将对应的移动向量设置为零
        // let position = self.base().get_position(); // 当前位置
        // if position.x <= 0.0 {
        //     
        // }
        // if position.x < 0.0 || position.x > world_size.x as f32 ||
        //     position.y < 0.0 || position.y > world_size.y as f32 {
        //     direction.x = 0.0;
        //     direction.y = 0.0;
        // }
        // 
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


        // todo 根据地图大小限制人物移动范围, 而不是通过空气墙
        
    }
}
