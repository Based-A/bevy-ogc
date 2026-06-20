//! `bevy-ogc` provides integration between [`ogc-rs`] and the [`bevy`] game engine.

#![no_std]

extern crate alloc;

mod input;

pub use input::*;

use bevy::app::plugin_group;

plugin_group! {
    /// This plugin group will add all the default plugins for a Bevy application using [`agb`].
    pub struct OgcPlugin {
        :OgcInputPlugin,
    }
}
