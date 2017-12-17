extern crate webmidi;
extern crate access_token_handler;

use webmidi::Access;
use access_token_handler::AccessKeyHandler as access_obj;

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

	let mut current_key = String::new();
    //3 simple test cases
    for x in 0..3 {
    	//No key
    	if x == 0 {
    		println!("Test Case {}", x+1);
    		current_key = String::from("");
    		println!("You don't have a key. We'll give you one.");
    	}
    	//Valid key
    	if x == 1 {
    		println!("Test Case {}", x+1);
    		current_key = access_obj::generate_key().key;
    	}
    	//Outdated key
    	if x == 2 {
    		println!("Test Case {}", x+1);
    		current_key = String::from("MTUxMzM2Njc0NyZhNTFhNjkxMy0wNmUyLTQ2ZDMtYWExMy00ODBmMzI4MzRkYWI=");
    	}

	    if current_key.len() > 0 {
	    	let valid = access_obj::check_key(&current_key).valid;
			if valid {
				println!("Your key is valid.");
			}
			else if !valid {
				println!("Key needs to be refreshed");
			}
	    }
	    else {
	    	println!("Your new key is: {}", access_obj::generate_key().key);
	    }
	}

}