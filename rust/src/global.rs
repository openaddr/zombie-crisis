use crate::player::Player;
use godot::prelude::*;
use godot::sys::Global;

/// 游戏全局数据
pub static GAME_DATA: Global<GameData> = Global::default();

#[derive(Default)]
pub struct GameData {
    /// 游戏世界大小(瓦片地图大小)
    pub world_size: Option<Vector2i>,
    
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GM {
    base: Base<Node>,

    pub players: Vec<Gd<Player>>,
}