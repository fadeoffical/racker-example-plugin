use racker::plugin::Plugin;
use racker::plugin::racker_plugin;

racker_plugin!(ExamplePlugin, ExamplePlugin::new);

pub struct ExamplePlugin;

impl ExamplePlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for ExamplePlugin {
    fn on_load(&self) {
        log::info!("Example plugin loaded");
    }

    fn on_unload(&self) {
        log::info!("Example plugin unloaded");
    }
}
