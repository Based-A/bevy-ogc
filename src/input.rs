use bevy::{
    input::gamepad::{RawGamepadAxisChangedEvent, RawGamepadButtonChangedEvent, RawGamepadEvent},
    prelude::*,
};

use ogc_rs::prelude::*;

#[derive(Default)]
pub struct OgcInputPlugin;

impl Plugin for OgcInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_controllers);
    }
    fn finish(&self, app: &mut App) {
        // Initialize controllers.
        Input::init(ControllerType::Gamecube);
        Input::init(ControllerType::Wii);
    }
}

// TODO: Rumble?
/// Check the buttons and stick inputs from all controllers.
/// This system runs in the PreUpdate schedule, and reads inputs from the OGC controller before writing them to their respective Bevy Gamepads for use in systems in the Update schedule.
fn update_controllers(
    controller_q: Query<(Entity, &OgcController)>,
    mut events: MessageWriter<RawGamepadEvent>,
    mut button_events: MessageWriter<RawGamepadButtonChangedEvent>,
    mut stick_events: MessageWriter<RawGamepadAxisChangedEvent>,
    mut prev_stick_values: Local<OgcSticks>,
) {
    Input::update(ControllerType::Gamecube);
    Input::update(ControllerType::Wii);

    for (entity, controller) in controller_q.iter() {
        // Process the button inputs for each controller.
        let held_buttons = controller.0.as_pad().buttons_held();
        let down_buttons = controller.0.as_pad().buttons_down();
        let up_buttons = controller.0.as_pad().buttons_up();

        // Iterate over all buttons returned in a category.
        held_buttons.iter().for_each(|button| {
            // Check to see the button will convert to a Bevy GamepadButton.
            if let Some(bevy_button) = pad_to_bevy_button(&button) {
                // Write the button event
                let event = RawGamepadButtonChangedEvent::new(entity, bevy_button, 1.);
                events.write(event.into());
                button_events.write(event);
            }
        });
        down_buttons.iter().for_each(|button| {
            if let Some(bevy_button) = pad_to_bevy_button(&button) {
                let event = RawGamepadButtonChangedEvent::new(entity, bevy_button, 1.);
                events.write(event.into());
                button_events.write(event);
            }
        });
        up_buttons.iter().for_each(|button| {
            if let Some(bevy_button) = pad_to_bevy_button(&button) {
                let event = RawGamepadButtonChangedEvent::new(entity, bevy_button, 0.);
                events.write(event.into());
                button_events.write(event);
            }
        });

        // Check if there's input to be read from the controller sticks.
        let stick_values = OgcSticks::read(&controller);

        // Check if the stick value have changed.
        if stick_values.main_x != prev_stick_values.main_x {
            // Create a new event if the value has changed.
            let left_stick_x_event = RawGamepadAxisChangedEvent::new(
                entity,
                GamepadAxis::LeftStickX,
                stick_values.main_x,
            );
            // Write the event to the event queues.
            events.write(left_stick_x_event.into());
            stick_events.write(left_stick_x_event);
            // Update the local stick value.
            prev_stick_values.main_x = stick_values.main_x;
        }
        if stick_values.main_y != prev_stick_values.main_y {
            let left_stick_y_event = RawGamepadAxisChangedEvent::new(
                entity,
                GamepadAxis::LeftStickY,
                stick_values.main_y,
            );
            events.write(left_stick_y_event.into());
            stick_events.write(left_stick_y_event);
            prev_stick_values.main_y = stick_values.main_y;
        }
        if stick_values.c_x != prev_stick_values.c_x {
            let right_stick_x_event =
                RawGamepadAxisChangedEvent::new(entity, GamepadAxis::RightStickX, stick_values.c_x);
            events.write(right_stick_x_event.into());
            stick_events.write(right_stick_x_event);
            prev_stick_values.c_x = stick_values.c_x;
        }
        if stick_values.c_y != prev_stick_values.c_y {
            let right_stick_y_event =
                RawGamepadAxisChangedEvent::new(entity, GamepadAxis::RightStickY, stick_values.c_y);
            events.write(right_stick_y_event.into());
            stick_events.write(right_stick_y_event);
            prev_stick_values.c_y = stick_values.c_y;
        }
    }
}

/// Helper component to access the Inputs from ogc_rs.
///
/// A new entity with this component should be created for every controller activated.
#[derive(Component, Deref, DerefMut)]
pub struct OgcController(ogc_rs::input::Input);

impl OgcController {
    #[must_use]
    pub fn new(controller_type: ControllerType, port: ControllerPort) -> Self {
        Self(ogc_rs::input::Input::new(controller_type, port))
    }
}

#[derive(Default)]
struct OgcSticks {
    main_x: f32,
    main_y: f32,
    c_x: f32,
    c_y: f32,
}

impl OgcSticks {
    /// Reads the stick values from the controller and returns a [`OgcSticks`] struct with values compatible with Bevy's [`GamepadAxis`].
    pub fn read(controller: &OgcController) -> Self {
        let pad = controller.0.as_pad();
        Self {
            main_x: (pad.stick_x() as f32) / (100.0),
            main_y: (pad.stick_y() as f32) / (100.0),
            c_x: (pad.c_stick_x() as f32) / (100.0),
            c_y: (pad.c_stick_y() as f32) / (100.0),
        }
    }
}

// TODO: How to work in Wii controller and Gamecube Controller variants?
/// Converts the bit values of an ogc-rs [`PadButton`] to a Bevy [`GamepadButton`].
const fn pad_to_bevy_button(
    button: &ogc_rs::input::pad::PadButton,
) -> Option<bevy::input::gamepad::GamepadButton> {
    match button.bits() {
        0x0001 => Some(GamepadButton::DPadLeft),
        0x0002 => Some(GamepadButton::DPadRight),
        0x0004 => Some(GamepadButton::DPadDown),
        0x0008 => Some(GamepadButton::DPadUp),
        0x0010 => Some(GamepadButton::Z),
        0x0020 => Some(GamepadButton::RightTrigger),
        0x0040 => Some(GamepadButton::LeftTrigger),
        0x0100 => Some(GamepadButton::South), // PAD_BUTTON_A
        0x0200 => Some(GamepadButton::West),  // PAD_BUTTON_B
        0x0400 => Some(GamepadButton::East),  // PAD_BUTTON_Y
        0x0800 => Some(GamepadButton::North), // PAD_BUTTON_X
        0x1000 => Some(GamepadButton::Start),
        _ => None,
    }
}
