use crate::global::{GAME_DATA, GM};
use crate::player::Player;
use godot::classes::{TileMapLayer, Timer};
use godot::prelude::*;
use crate::game_manager;

#[derive(GodotClass)]
#[class(init,base=Node)]
struct MainScene {
    base: Base<Node>,

    #[init(node = "/root/gm")]
    gm: OnReady<Gd<GM>>,

    #[init(node = "Player")]
    player: OnReady<Gd<Player>>,

    #[init(node = "Road")]
    road: OnReady<Gd<TileMapLayer>>,
}

impl MainScene {
    fn spawn_enemy(&mut self) {
        godot_print!("spawn_enemy")
    }
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
        GAME_DATA.lock().world_size  = Some(world_size);
        // self.gm.bind_mut().world_size = Some(world_size);
        godot_print!("设置世界大小完毕");
        // let mut gd = Player::new_alloc();
        // gd.set_position(Vector2::new(51.0, 90.0));
        // gd_mut.base_mut().add_child(&gd);
        // godot_print!("添加玩家完毕")

        let mut timer = Timer::new_alloc();

        timer.set_wait_time(1.0);
        let mut gd = self.to_gd();
        let local_fn = Callable::from_local_fn("hh", move |_| {
            gd.bind_mut().spawn_enemy();
            Ok(Variant::nil())
        });
        timer.set_one_shot(false);
        timer.connect("timeout", &local_fn);
        timer.set_autostart(true);
        // timer.start();
        self.base_mut().add_child(&timer);
        // self.base_mut().

        godot_print!("添加定时器完毕");
    }
}
