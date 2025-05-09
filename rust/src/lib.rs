use godot::prelude::{ExtensionLibrary, gdextension};

mod const_data;
mod enemy;
mod ez;
mod global;
mod main_scene;
mod player;

struct Ext;

#[gdextension]
unsafe impl ExtensionLibrary for Ext {}
