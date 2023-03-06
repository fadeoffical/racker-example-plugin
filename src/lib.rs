use racker_plugin::{PluginMeta, racker_plugin};
use racker_plugin::Plugin;

racker_plugin!(ExamplePlugin, ExamplePlugin::new);

pub struct ExamplePlugin;

impl ExamplePlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for ExamplePlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: String::from("racker-example-plugin"),
            version: String::from("0.1.0"),
            description: String::from("An example racker plugin"),
            author: String::from("fade"),
        }
    }

    fn on_load(&self) {
        log::info!("Example plugin loaded");
    }

    fn on_unload(&self) {
        log::info!("Example plugin unloaded");
    }
}
