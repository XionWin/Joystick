use std::ops::IndexMut;

use crate::{Gamepad, control::Control};

use joystick_core as jsc;

pub fn update_with_init_events(gamepad: &mut Gamepad, events: Vec<jsc::event::Event>) {
    for event in events {
        match event.type_ {
            v if v == jsc::event::EventType::EventButtonInit as u8 => {
                update_button(gamepad, event);
            },
            v if v == jsc::event::EventType::EventAxisInit as u8 => {
                update_axis(gamepad, event);
            },
            _ => {}
        }
    }
}

fn update_button(gamepad: &mut Gamepad, event: jsc::event::Event) {
    gamepad.buttons.index_mut(event.number as usize).set_value(event.value);
}

fn update_axis(gamepad: &mut Gamepad, event: jsc::event::Event) {
    gamepad.axes.index_mut(event.number as usize).set_value(event.value);
}
