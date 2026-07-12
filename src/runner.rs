use bevy::{app::PluginsState, prelude::*};

use ogc_rs::prelude::*;

#[derive(Default)]
pub struct OgcRunnerPlugin;

impl Plugin for OgcRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(|mut app| {
            let video = Video::init();
            let mut asnd = Asnd::init();

            Input::init(ControllerType::Gamecube);
            Input::init(ControllerType::Wii);

            Console::init(&video);
            Video::configure(&video.render_config);
            unsafe {
                Video::set_next_framebuffer(video.framebuffer);
            }
            Video::set_black(false);
            Video::flush();

            while app.plugins_state() == PluginsState::Adding {}

            app.finish();
            app.cleanup();

            loop {
                app.update();

                if let Some(exit) = app.should_exit() {
                    return exit;
                }

                Video::wait_vsync();
            }
        });
    }
}
