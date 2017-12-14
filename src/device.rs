extern crate midir;

use midir::{MidiInput, MidiOutput, PortInfoError};
use std::marker::PhantomData;

#[derive(Debug)]
pub enum DeviceState {
    DISCONNECTED,
    CONNECTED
}

#[derive(Debug)]
pub enum DeviceConnectionState {
    OPEN,
    CLOSED,
    PENDING
}

// Hack for specifying the types to use in Device
pub trait Midi {
    fn _port_name(&self, port_number: usize) -> Result<String, PortInfoError>;
}
impl Midi for MidiInput {
    fn _port_name(&self, port_number: usize) -> Result<String, PortInfoError> {
        return self.port_name(port_number);
    }
}
impl Midi for MidiOutput {
    fn _port_name(&self, port_number: usize) -> Result<String, PortInfoError> {
        return self.port_name(port_number);
    }
}

pub struct Device<T> {
    _phantom: PhantomData<T>,
    port: usize,
    name: String,
    state: DeviceState,
    connection_state: DeviceConnectionState
}

impl<T: Midi> Device<T> {
    pub fn new(midi: &T, port: usize) -> Device<T> {
        let name = midi._port_name(port)
            .expect(&format!("Failed to get MIDI Device name for port {}", port));

        return Self {
            _phantom: PhantomData,
            port,
            name,
            state: DeviceState::CONNECTED,
            connection_state: DeviceConnectionState::CLOSED
        };
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }

    pub fn state(&self) -> &DeviceState {
        return &self.state;
    }

    pub fn connection_state(&self) -> &DeviceConnectionState {
        return &self.connection_state;
    }
}

impl Device<MidiInput> {
    pub fn send(&self) {
        println!("MAYBE???");
    }

    pub fn port_type(&self) -> &str {
        return "input";
    }
}

impl Device<MidiOutput> {
    pub fn read(&self) {
        println!("MAYBE???");
    }

    pub fn port_type(&self) -> &str {
        return "output";
    }
}