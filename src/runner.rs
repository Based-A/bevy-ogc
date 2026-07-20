use bevy::{app::PluginsState, prelude::*};

use ogc_rs::prelude::*;
use ogc_rs::video::Video;

/// A custom runner that sets up the various OGC components needed to run on the hardware.
#[derive(Default)]
pub struct OgcRunnerPlugin;

impl Plugin for OgcRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(|mut app| {
            // Wait for Plugins to be added to the app.
            while app.plugins_state() == PluginsState::Adding {}

            // Call the remaining steps of all Plugins.
            app.finish();
            app.cleanup();

            // Begin the application loop.
            loop {
                app.update();

                if let Some(exit) = app.should_exit() {
                    return exit;
                }

                // Wait on the hardware to reach VSync before starting the next frame loop.
                Video::wait_vsync();
            }
        });
    }
}
