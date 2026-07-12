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
}

/// Check the buttons and stick inputs from all controllers.
/// This system runs in the PreUpdate schedule, and reads inputs from
/// the OGC controller before then writing them to every Gamepad in
/// Bevy for use in the Update schedule.
/// TODO: Rumble?
fn update_controllers(
    controller_q: Query<(Entity, &OgcController)>,
    mut events: MessageWriter<RawGamepadEvent>,
    mut button_events: MessageWriter<RawGamepadButtonChangedEvent>,
    mut stick_events: MessageWriter<RawGamepadAxisChangedEvent>,
) {
    Input::update(ControllerType::Gamecube);
    Input::update(ControllerType::Wii);

    let input_flags: Vec<Button> = vec![
        Button::Left,
        Button::Right,
        Button::Up,
        Button::Down,
        Button::TrigL,
        Button::TrigR,
        Button::TrigZ,
        Button::TrigZL,
        Button::TrigZR,
        Button::A,
        Button::B,
        Button::C,
        Button::X,
        Button::Y,
        Button::Z,
        Button::One,
        Button::Two,
        Button::Minus,
        Button::Plus,
        Button::Home,
        Button::Start,
    ];

    for (entity, controller) in controller_q.iter() {
        // Process the button inputs for each controller.
        // TODO: Currently alter the upstream crate by deriving `Clone` and `Copy` on Button. Will have to see about upstreaming.
        input_flags
            .iter()
            .filter_map(ogc_to_bevy_button)
            .filter_map(|(ogc_button, bevy_button)| {
                controller
                    .0
                    .is_button_held(ogc_button)
                    .then_some(1.)
                    .or(controller.0.is_button_up(ogc_button).then_some(0.))
                    .map(|value| RawGamepadButtonChangedEvent::new(entity, bevy_button, value))
            })
            .for_each(|event| {
                events.write(event.into());
                button_events.write(event);
            });

        // Check if there's input to be read from the controller sticks.
        // TODO: Has to be a better way to do this, lol.
        if let Some(left_stick_x_input) = controller.as_pad().to_bevy_stick(GamepadAxis::LeftStickX)
        {
            let left_stick_x_event = RawGamepadAxisChangedEvent::new(
                entity,
                GamepadAxis::LeftStickX,
                left_stick_x_input,
            );
            events.write(left_stick_x_event.into());
            stick_events.write(left_stick_x_event);
        }
        if let Some(left_stick_y_input) = controller.as_pad().to_bevy_stick(GamepadAxis::LeftStickX)
        {
            let left_stick_y_event = RawGamepadAxisChangedEvent::new(
                entity,
                GamepadAxis::RightStickY,
                left_stick_y_input,
            );
            events.write(left_stick_y_event.into());
            stick_events.write(left_stick_y_event);
        }
        if let Some(right_stick_x_input) =
            controller.as_pad().to_bevy_stick(GamepadAxis::RightStickX)
        {
            let right_stick_x_event = RawGamepadAxisChangedEvent::new(
                entity,
                GamepadAxis::RightStickX,
                right_stick_x_input,
            );
            events.write(right_stick_x_event.into());
            stick_events.write(right_stick_x_event);
        }
        if let Some(right_stick_y_input) =
            controller.as_pad().to_bevy_stick(GamepadAxis::RightStickY)
        {
            let right_stick_y_event = RawGamepadAxisChangedEvent::new(
                entity,
                GamepadAxis::RightStickY,
                right_stick_y_input,
            );
            events.write(right_stick_y_event.into());
            stick_events.write(right_stick_y_event);
        }
    }
}

/// Helper component to access the Inputs from ogc_rs.
#[derive(Component, Deref, DerefMut)]
pub struct OgcController(ogc_rs::input::Input);

impl OgcController {
    #[must_use]
    pub fn new(controller_type: ControllerType, port: i8) -> Self {
        let port_num = match port {
            0 => ControllerPort::One,
            1 => ControllerPort::Two,
            2 => ControllerPort::Three,
            3 => ControllerPort::Four,
            _ => ControllerPort::One,
        };
        Self(ogc_rs::input::Input::new(controller_type, port_num))
    }
}

trait OgcStick {
    // Conversion from Ogc Stick to a Bevy Stick
    fn to_bevy_stick(&self, stick: GamepadAxis) -> Option<f32>;
}

impl OgcStick for ogc_rs::input::Pad {
    fn to_bevy_stick(&self, stick: GamepadAxis) -> Option<f32> {
        use bevy::input::gamepad::GamepadAxis;

        match stick {
            GamepadAxis::LeftStickX => {
                if self.stick_x() == 0 {
                    return None;
                }
                return Some(self.stick_y() as f32);
            }
            GamepadAxis::LeftStickY => {
                if self.stick_y() == 0 {
                    return None;
                }
                return Some(self.stick_y() as f32);
            }
            GamepadAxis::RightStickX => {
                if self.c_stick_x() == 0 {
                    return None;
                }
                return Some(self.c_stick_y() as f32);
            }
            GamepadAxis::RightStickY => {
                if self.c_stick_y() == 0 {
                    return None;
                }
                return Some(self.c_stick_y() as f32);
            }
            _ => return None,
        }
    }
}

const fn ogc_to_bevy_button(
    button: &ogc_rs::input::Button,
) -> Option<(ogc_rs::input::Button, bevy::input::gamepad::GamepadButton)> {
    use ogc_rs::input::Button;

    match button {
        Button::Left => Some((Button::Left, GamepadButton::DPadLeft)),
        Button::Right => Some((Button::Right, GamepadButton::DPadRight)),
        Button::Up => Some((Button::Up, GamepadButton::DPadUp)),
        Button::Down => Some((Button::Down, GamepadButton::DPadDown)),
        Button::TrigL => Some((Button::TrigL, GamepadButton::LeftTrigger)),
        Button::TrigR => Some((Button::TrigR, GamepadButton::RightTrigger)),
        Button::TrigZ => Some((Button::TrigZ, GamepadButton::Z)),
        Button::TrigZL => Some((Button::TrigZL, GamepadButton::LeftTrigger2)),
        Button::TrigZR => Some((Button::TrigZR, GamepadButton::RightTrigger2)),
        Button::A => Some((Button::A, GamepadButton::East)),
        Button::B => Some((Button::B, GamepadButton::South)),
        Button::C => Some((Button::C, GamepadButton::C)),
        Button::X => Some((Button::X, GamepadButton::North)),
        Button::Y => Some((Button::Y, GamepadButton::West)),
        Button::Z => Some((Button::Z, GamepadButton::Z)),
        Button::One => Some((Button::One, GamepadButton::Other(0))),
        Button::Two => Some((Button::Two, GamepadButton::Other(1))),
        Button::Minus => Some((Button::Minus, GamepadButton::Other(2))),
        Button::Plus => Some((Button::Plus, GamepadButton::Other(3))),
        Button::Home => Some((Button::Home, GamepadButton::Mode)),
        Button::Start => Some((Button::Start, GamepadButton::Start)),
    }
}
