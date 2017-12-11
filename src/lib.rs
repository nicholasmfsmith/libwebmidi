extern crate midir;

use std::error::Error;
use std::collections::HashMap;
use midir::{MidiInput, MidiOutput};

// Input
pub struct WebMidiInput {
    handle: String
}

impl WebMidiInput {
    pub fn new(handle: String) -> WebMidiInput {
        WebMidiInput { handle: handle }
    }

    pub fn inputs(&self) -> Result<HashMap<usize, String>, Box<Error>> {
        let midi_in = MidiInput::new(&self.handle)?;
        let mut map = HashMap::new();

        for i in 0..midi_in.port_count() {
            let name = midi_in.port_name(i)?;
            map.insert(i, name);
        }

        return Ok(map);
    }
}

// Output
pub struct WebMidiOutput {
    handle: String
}

impl WebMidiOutput {
    pub fn new(handle: String) -> WebMidiOutput {
        WebMidiOutput { handle: handle }
    }

    pub fn outputs(&self) -> Result<HashMap<usize, String>, Box<Error>> {
        let midi_out = MidiOutput::new(&self.handle)?;
        let mut map = HashMap::new();

        for i in 0..midi_out.port_count() {
            let name = midi_out.port_name(i)?;
            map.insert(i, name);
        }

        return Ok(map);
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
