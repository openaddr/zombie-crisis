use crate::const_data::Direction;
use crate::global::{global, Global};
use godot::classes::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,

    #[init(node = "/root/TheGlobal")]
    global: OnReady<Gd<Global>>,

    #[init(val = 100.0)]
    speed: f32,

    direction: Direction,

    #[init(node = "AnimatedSprite2D")]
    animator: OnReady<Gd<AnimatedSprite2D>>,

    #[init(node = "Camera2D")]
    camera: OnReady<Gd<Camera2D>>,
    
    name : String,
}

impl Player {
    ///  改变相机边界
    fn change_camera_limit(&mut self) {
        let world_size = self.global.bind().world_size.expect("地图大小值尚未初始化");
        self.camera.set_limit(Side::TOP, 0); // 每帧设置世界边界, todo 太浪费性能, 是否能优化
        self.camera.set_limit(Side::LEFT, 0);
        self.camera.set_limit(Side::BOTTOM, world_size.y);
        self.camera.set_limit(Side::RIGHT, world_size.x);
    }

    /// 判断移动方向, 如果没有移动方向, 则暂停动画, 否则根据移动方向播放动画
    fn change_animation_by_direction(&mut self, direction: &Vector2) {
        let direction_state = Direction::from_direction_vector_vector(direction); // 获取移动方向
        let Some(direct) = direction_state else {
            self.animator.pause(); // 如果静止, 暂停动画
            return;
        };
        // 如果有移动方向, 就播放动画
        self.animator.play();
        if direct == self.direction {
            // 如果移动方向没有改变, 则返回
            return;
        }
        self.direction = direct; // 设置当前方向
        self.change_animation_by_direct();
    }

    /// 根据方向改变动画, 因为动画名与方向是一一对应的, 所以直接设置动画名即可
    fn change_animation_by_direct(&mut self) {
        self.animator.set_animation(self.direction.as_str());
    }

    /// 设置速度, 并移动
    fn set_v_and_move(&mut self, direction: Vector2) {
        let v = direction * self.speed; // 矢量速度
        self.base_mut().set_velocity(v);
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn physics_process(&mut self, _: f64) {
        let direction = Input::singleton().get_vector("ui_left", "ui_right", "ui_up", "ui_down");
        self.change_camera_limit(); // 改变相机边界
        self.set_v_and_move(direction);
        self.change_animation_by_direction(&direction);
    }

    fn ready(&mut self) {
        let mut gd = global();
        let mut gd = gd.bind_mut();
        gd.players.push(self.to_gd());
    }
}
