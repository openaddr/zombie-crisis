use godot::prelude::{ExtensionLibrary, gdextension};

mod main_scene;
mod player;
mod const_data;

struct Ext;

#[gdextension]
unsafe impl ExtensionLibrary for Ext {}
