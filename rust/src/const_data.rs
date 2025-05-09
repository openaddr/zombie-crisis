use godot::prelude::Vector2;

/// 方向朝向, 默认朝下
#[derive(Default,PartialEq)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl Direction {
    /// 根据方向向量, 确定正在移动的方向; 如果方向向量为零向量(静止), 则返回None
    /// 优先考虑水平方向
    pub fn from_direction_vector_vector(direction_vector: &Vector2) -> Option<Self> {
        if direction_vector.x > 0.0 {
            Some(Direction::Right)
        } else if direction_vector.x < 0.0 {
            Some(Direction::Left)
        } else if direction_vector.y > 0.0 {
            Some(Direction::Down)
        } else if direction_vector.y < 0.0 {
            Some(Direction::Up)
        } else {
            None // 如果没有移动向量,则返回None,代表静止
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match *self {
            // *self 有 Direction 类型
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}
