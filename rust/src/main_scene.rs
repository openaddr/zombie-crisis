use crate::global::global;
use crate::player::Player;
use godot::classes::TileMapLayer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Node)]
struct MainScene {
    base: Base<Node>,

    #[init(node = "Player")]
    player: OnReady<Gd<Player>>,
    #[init(node = "Road")]
    road: OnReady<Gd<TileMapLayer>>,
}

#[godot_api]
impl INode for MainScene {
    fn ready(&mut self) {
        godot_print!("ready");
        let rect2i = self.road.get_used_rect();
        let i = self.road.get_rendering_quadrant_size();
        let world_size = rect2i.size * i;
        godot_print!("{:?} ", world_size);
        godot_print!("rect2i.end() {:?} ", rect2i.end());
        godot_print!("rect2i.position {:?} ", rect2i.position);

        let mut global = global();
        let mut gd_mut = global.bind_mut();
        gd_mut.world_size = Some(world_size);
        godot_print!("设置世界大小完毕");
        // let mut gd = Player::new_alloc();
        // gd.set_position(Vector2::new(51.0, 90.0));
        // gd_mut.base_mut().add_child(&gd);
        // godot_print!("添加玩家完毕")
    }
}
