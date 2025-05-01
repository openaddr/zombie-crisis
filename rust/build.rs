use std::io::Result;
use gdext_gen::prelude::*;
fn main() -> Result<()> {
    generate_gdextension_file(
        BaseDirectory::ProjectFolder,
        Some("../rust/target".into()),
        Some("../godot/rust.gdextension".into()),
        false,
        Some(Configuration::new(
            EntrySymbol::GodotRustDefault,
            Some((4, 3)),
            None,
            true,
            false,
        )),
        Some(WindowsABI::MSVC),
        Some(IconsConfig::new(
            DefaultNodeIcon::NodeRust(NodeRust::Large, "rust".into()),
            IconsCopyStrategy::new(true, true, "../godot/addons/rust".into(), false),
            None,
            IconsDirectories::new("addons".into(), "editor".into(), "rust".into(), BaseDirectory::ProjectFolder.into()),
        )),
        None,
    )?;
    
    Ok(())
}