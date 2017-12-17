extern crate security_midi;

fn main() {
    let mut current_key = String::new();
    //3 simple test cases
    for x in 0..3 {
    	//No key
    	if x == 0 {
    		current_key = String::from("");
    		println!("You don't have a key. We'll give you one.");
    	}
    	//Valid key
    	if x == 1 {
    		current_key = security_midi::AccessKeyHandler::generate_key();
    	}
    	//Outdated key
    	if x == 2 {
    		current_key = String::from("MTUxMzM2Njc0NyZhNTFhNjkxMy0wNmUyLTQ2ZDMtYWExMy00ODBmMzI4MzRkYWI=");
    	}

	    if current_key.len() > 0 {
			security_midi::AccessKeyHandler::check_key(&current_key);
	    }
	    else {
	    	println!("Your new key is: {}", security_midi::AccessKeyHandler::generate_key());
	    }
	}
}