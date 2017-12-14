extern crate webmidi;

use webmidi::Access;

fn main() {
	let acc = Access::new(0);

	println!("Available MIDI Inputs");
	for (port, input) in acc.inputs() {
		println!("{}: {} ({:?})", port, input.name(), input.state());
	}

	println!("Available MIDI Outputs");
	for (port, output) in acc.outputs() {
		println!("{}: {} ({:?})", port, output.name(), output.state());
	}
}