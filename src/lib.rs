//! `bevy-ogc` provides integration between [`ogc-rs`] and the [`bevy`] game engine.

#![no_std]

extern crate alloc;

//mod error;
//mod network;
//mod console;
//mod debug;
//mod time;
mod audio;
mod gx;
mod input;
mod runner;
mod video;

pub use audio::*;
pub use gx::*;
pub use input::*;
pub use runner::*;
pub use video::*;

use bevy::app::plugin_group;

plugin_group! {
    /// This plugin group will add all the default plugins for a Bevy application using [`ogc-rs`].
    pub struct OgcPlugin {
        :OgcRunnerPlugin,
        :OgcInputPlugin,
        :OgcVideoPlugin,
        :OgcGxPlugin,
        :OgcAudioPlugin,
    }
}
