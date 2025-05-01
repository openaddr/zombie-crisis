use godot::prelude::{ExtensionLibrary, gdextension};

mod main_scene;
mod player;

struct Ext;

#[gdextension]
unsafe impl ExtensionLibrary for Ext {}
