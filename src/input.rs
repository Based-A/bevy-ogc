use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonStateChangedEvent,
        GamepadConnection, GamepadConnectionEvent, GamepadEvent, RawGamepadAxisChangedEvent,
        RawGamepadButtonChangedEvent, RawGamepadEvent,
    },
    prelude::*,
};

use ogc_rs::prelude::*;

#[derive(Default)]
pub struct OgcInputPlugin;

impl Plugin for OgcInputPlugin {
    fn build(&self, app: &mut App) {
        Input::init(ControllerType::Gamecube);
        Input::init(ControllerType::Wii);

        let gcn_ctrl = Input::new(ControllerType::Gamecube, ControllerPort::One);
        let wii_ctrl = Input::new(ControllerType::Wii, ControllerPort::One);
    }

    fn finish(&self, app: &mut App) {
        Input::update(ControllerType::Gamecube);
        Input::update(ControllerType::Wii);
    }
}
