extern crate webmidi;

fn main() {
	let wmi = webmidi::WebMidiInput::new(String::from("Test input Stream"));

	match wmi.inputs() {
		Ok(inputs) => {
			println!("Available MIDI Inputs");
			for (port, name) in &inputs {
				println!("{}: {}", port, name);
			}
		},
		Err(err) => println!("Error: {}", err.description())
	}

	let wmo = webmidi::WebMidiOutput::new(String::from("Test output Stream"));

	match wmo.outputs() {
		Ok(outputs) => {
			println!("Available MIDI Outputs");
			for (port, name) in &outputs {
				println!("{}: {}", port, name);
			}
		},
		Err(err) => println!("Error: {}", err.description())
	}
}