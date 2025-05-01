use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Node)]
struct MainScene {
    base: Base<Node>
}

#[godot_api]
impl INode for MainScene {
    
}
