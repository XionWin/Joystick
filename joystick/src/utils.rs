use std::ops::IndexMut;

use crate::{Gamepad, control::Control, EventType, JsEvent};

use joystick_core as jsc;

pub fn update_with_init_events(gamepad: &mut Gamepad, events: Vec<jsc::js::linux::Event>) {
    for event in events {
        match event.type_ {
            v if v == jsc::js::linux::EventType::EventButtonInit as u8 => {
                update_button(gamepad, event);
            },
            v if v == jsc::js::linux::EventType::EventAxisInit as u8 => {
                update_axis(gamepad, event);
            },
            _ => {}
        }
    }
}

pub fn update_with_events(gamepad: &mut Gamepad, event: jsc::js::linux::Event) -> JsEvent {
    match event.type_ {
        v if v == jsc::js::linux::EventType::EventButtonInit as u8 || v == jsc::js::linux::EventType::EventButton as u8 => {
            update_button(gamepad, event)
        },
        v if v == jsc::js::linux::EventType::EventAxisInit as u8 || v == jsc::js::linux::EventType::EventAxis as u8 => {
            update_axis(gamepad, event)
        },
        _ => JsEvent::default()
    }
}

fn update_button(gamepad: &mut Gamepad, event: jsc::js::linux::Event) -> JsEvent {
    let button = gamepad.buttons.index_mut(event.number as usize);
    button.set_value(event.value);
    JsEvent::new(
        match event.type_ {
            v if v == jsc::js::linux::EventType::EventButtonInit as u8 => EventType::ButtonInit,
            v if v == jsc::js::linux::EventType::EventButton as u8 => EventType::Button,
            _ => EventType::Unknown
        },
        button.get_id(),
        button.get_alias(),
        button.get_value()
    )
}

fn update_axis(gamepad: &mut Gamepad, event: jsc::js::linux::Event) -> JsEvent {
    let axis = gamepad.axes.index_mut(event.number as usize);
    axis.set_value(event.value);
    JsEvent::new(
        match event.type_ {
            v if v == jsc::js::linux::EventType::EventAxisInit as u8 => EventType::AxisInit,
            v if v == jsc::js::linux::EventType::EventAxis as u8 => EventType::Axis,
            _ => EventType::Unknown
        },
        axis.get_id(),
        axis.get_alias(),
        axis.get_value()
    )
}
