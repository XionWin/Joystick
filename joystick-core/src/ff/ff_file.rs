use std::os::unix::prelude::RawFd;

use nix::io::{def::OpenMode, File};

use crate::{RumbleEffect, UEffect};

use super::ff_utils;


#[derive(Debug)]
pub struct FfFile {
    file: File
}

impl FfFile {
    pub fn new(path: &str) -> Self  {
        FfFile {
            file: File::new(path)
        }
    }

    pub fn get_fd(&self) -> RawFd {
        self.file.get_fd()
    }

    pub fn is_connected(&self) -> bool {
        self.file.is_connected()
    }
    
    pub fn open(mut self, mode: OpenMode) -> Self {
        self.file = self.file.open(mode);
        self
    }

    pub fn close(&mut self) {
        self.file.close();
    }

    pub fn switch(self, mode: OpenMode) {
        self.file.switch(mode);
    }

    pub fn set_rumble_effect(&self) -> Result<u16, &'static str> {
        let effect = UEffect {
            rumble: RumbleEffect {
                strong_magnitude: 0x8000,
                weak_magnitude: 0,
            }
        };
        ff_utils::set_effect(self.get_fd(), crate::EffectType::Rumble, effect)
    }


    pub fn run_effect(&self, id: u16) -> bool {
        ff_utils::run_effect(self.get_fd(), id)
    }

    pub fn remove_effect(&self, id: u16) -> bool {
        ff_utils::remove_effect(self.get_fd(), id)
    }

    // pub fn read_driver_version(&self) -> Result<u32, &'static str> {
    //     js_utils::read_driver_version(self.fd())
    // }
    
    // pub fn read_axis_count(&self) -> Result<u8, &'static str> {
    //     js_utils::read_axis_count(self.fd())
    // }
    
    // pub fn read_button_count(&self) -> Result<u8, &'static str> {
    //     js_utils::read_button_count(self.fd())
    // }

    // pub fn read_axis_mapping(&self, size: usize) -> Result<Vec<Axis>, &'static str> {
    //     js_utils::read_axis_mapping(self.fd(), size)
    // }

    // pub fn read_button_mapping(&self, size: usize) -> Result<Vec<Key>, &'static str> {
    //     js_utils::read_button_mapping(self.fd(), size)
    // }

    // pub fn read_name(&self) -> Result<String, &'static str> {
    //     js_utils::read_name(self.fd())
    // }

    // pub fn read_event_with_block(&self) -> Event {
    //     js_utils::read_event_with_block(self.fd())
    // }

    // pub fn read_init_event_with_no_block(&self) -> Vec<Event> {
    //     js_utils::read_init_event_with_no_block(self.fd())
    // }
}