
#[macro_export]
macro_rules! game_manager {
    ($self:expr) => {{
        $self.base().get_node_as::<GM>("/root/gm")
    }};
}