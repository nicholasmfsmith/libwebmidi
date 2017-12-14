extern crate midir;

mod device;

use std::collections::HashMap;
use midir::{MidiInput, MidiOutput, Ignore};

type Token = i32; // TODO: Wait for security object from Smith
type DeviceMap<T> = HashMap<usize, device::Device<T>>;
type Handler = i32; // CHANGE to some sort of closure

pub struct Access {
    token: Token,
    inputs: DeviceMap<MidiInput>,
    outputs: DeviceMap<MidiOutput>,
    on_state_change: Handler,
    sysex_enabled: bool,

    _midi_in_handle: MidiInput,
    _midi_out_handle: MidiOutput
}

impl Access {
    pub fn new(token: Token) -> Access {
        // if (token.isValid())
        let has_sysex: bool = false; // token.hasSysex()

        let mut _in  = MidiInput::new("Web Midi IN").expect("Failed to open MIDI Input Stream");
        let mut _out = MidiOutput::new("Web Midi OUT").expect("Failed to open MIDI Output Stream");

        // Ignore sysex messages if not allowed
        if has_sysex {
            _in.ignore(Ignore::None);
        } else {
            _in.ignore(Ignore::Sysex);
        }

        let mut _in_map = HashMap::new();
        let mut _out_map = HashMap::new();

        // Get Devices
        for i in 0.._in.port_count() {
            let ins_in = device::Device::new(&_in, i);
            _in_map.insert(i, ins_in);
        }

        for i in 0.._out.port_count() {
            let ins_out = device::Device::new(&_out, i);
            _out_map.insert(i, ins_out);
        }

        return Access {
            token: token,
            inputs: _in_map,
            outputs: _out_map,
            on_state_change: 0,
            sysex_enabled: has_sysex,

            _midi_in_handle: _in,
            _midi_out_handle: _out
        }
    }

    pub fn inputs(&self) -> &DeviceMap<MidiInput> {
        return &self.inputs;
    }

    pub fn outputs(&self) -> &DeviceMap<MidiOutput> {
        return &self.outputs;
    }
}

// Testing section
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        list();
        assert_eq!(2 + 2, 4);
    }
}
