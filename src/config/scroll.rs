use hidapi::HidDevice;
use crate::args::{Direction, Button, Binding, MouseFn};
use super::bind;

pub fn set(device: &HidDevice, direction: Direction) {
    for i in 1..=3 {
        match direction {
            Direction::Default => {
                // Up => Up
                bind::set(
                    device, Some(i),
                    Button::ScrollUp,
                    Binding::Mouse(MouseFn::ScrollUp)
                );

                // Down => Down
                bind::set(
                    device, Some(i),
                    Button::ScrollDown,
                    Binding::Mouse(MouseFn::ScrollDown)
                );
            },

            Direction::Invert => {
                // Up => Down
                bind::set(
                    device, Some(i),
                    Button::ScrollUp,
                    Binding::Mouse(MouseFn::ScrollDown)
                );

                // Down => Up
                bind::set(
                    device, Some(i),
                    Button::ScrollDown,
                    Binding::Mouse(MouseFn::ScrollUp)
                );
            },
        }
    }
}