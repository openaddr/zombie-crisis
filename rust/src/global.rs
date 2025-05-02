use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Global {
    base: Base<Node>,
    
    #[init(val="hello")]
   pub string: &'static str,
    
    #[init(val=100)]
   pub int: i64
}
