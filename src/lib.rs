//! `bevy-ogc` provides integration between [`ogc-rs`] and the [`bevy`] game engine.

#![no_std]

extern crate alloc;

//mod ios;
//mod error;
//mod network;
//mod audio
//mod mp3;
//mod console;
//mod debug;
//mod utils;
//mod gu;
//mod runtime;
mod gx;
//mod asnd;
//mod aesnd;
mod input;
//mod lwp;
//mod mutex;
//mod cache;
//mod tpl;
//mod time;
//mod ffi;
//mod mmio;
mod runner;

pub use gx::*;
pub use input::*;
pub use runner::*;

use bevy::app::plugin_group;

plugin_group! {
    /// This plugin group will add all the default plugins for a Bevy application using [`ogc-rs`].
    pub struct OgcPlugin {
        :OgcRunnerPlugin,
        :OgcInputPlugin,
        :OgcGxPlugin,
    }
}
