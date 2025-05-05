use crate::player::Player;
use godot::classes::Engine;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Global {
    base: Base<Node>,

    /// 游戏世界大小(瓦片地图大小)
    pub world_size: Option<Vector2i>,

    pub players: Vec<Gd<Player>>,
}

pub fn global() -> Gd<Global> {
    Engine::singleton()
        .get_singleton("Global")
        .unwrap()
        .cast::<Global>()
}
