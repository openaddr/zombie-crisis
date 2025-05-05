use crate::global::Global;
use godot::classes::Engine;
use godot::init::InitLevel;
use godot::obj::NewAlloc;
use godot::prelude::{gdextension, godot_error, ExtensionLibrary};

mod const_data;
mod global;
mod main_scene;
mod player;
mod enemy;

struct Ext;

#[gdextension]
unsafe impl ExtensionLibrary for Ext {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // `&str` 用于标识您的单例，稍后可以用它来访问单例。
            Engine::singleton().register_singleton("Global", &Global::new_alloc());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // 保留我们的引擎单例实例 和 Global名称 变量。
            let mut engine = Engine::singleton();
            let singleton_name = "Global";

            // 这里，我们手动检索已注册的单例，
            // 以便注销它们并释放内存 —— 注销单例不会自动由库处理。
            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                // 注销 Godot 中的单例并释放内存，以避免内存泄漏、警告和热重载问题。
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                // 在这里，您可以选择恢复或触发 panic。
                godot_error!("Failed to get singleton");
                panic!("应该获取到单例的, 但没找到");
            }
        }
    }
}
