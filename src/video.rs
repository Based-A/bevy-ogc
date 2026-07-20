use bevy::prelude::*;

use ogc_rs::video::Video;

#[derive(Default)]
pub struct OgcVideoPlugin;

impl Plugin for OgcVideoPlugin {
    fn build(&self, app: &mut App) {
        // (Maybe unnecessary, might be able to get away with placing everything in `build`)
        // Make sure that the user calls Video::init() and places it in a non_send resource
        // before initializing the rest of the plugins.
        // This is because as of 0.19, there is no way to order plugins, and Video::init() must
        // be placed in the app for the rest of the plugins to function.

        // Initialize the system, place reference to the Video Config in a resource.
        // Video has to be created first in order for the Gamecube hardware to function.
        let video = Video::init();
        app.insert_non_send::<OgcVideo>(OgcVideo(video));
    }
    fn finish(&self, app: &mut App) {
        // Get a reference to the Video struct.
        let video = app.world().non_send::<OgcVideo>();
        // Ensure the preferred settings are applied.
        Video::configure(&video.render_config);

        // Indicate that this buffer will be drawn on during the next draw step.
        unsafe { Video::set_next_framebuffer(video.framebuffer) };

        // Reveal what is supposed to be on the screen.
        Video::set_black(false);
        // Accept the Video changes that have been made.
        Video::flush();
    }
}

/// Resource to hold the OGC Video state.
/// Initialized and inserted first in the application.
#[derive(Deref, DerefMut)]
pub struct OgcVideo(pub Video);
