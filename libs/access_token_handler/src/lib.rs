extern crate uuid;
extern crate base64;

pub struct AccessKeyHandler {
	pub valid: bool,
	pub key: String
}


impl AccessKeyHandler {
	//Check validity of existing key (valid condition: key must be < 86400 seconds old)
	pub fn check_key(existing_key: &str) -> AccessKeyHandler {
		println!("checking key");
		println!("check_key (key) {}", existing_key);
		let b = base64::decode(&existing_key).unwrap();
		let result = std::str::from_utf8(&b).unwrap();
		println!("decoded: {}", result);

		let provided_epoch: Vec<&str> = result.split("&").collect();

		println!("provided_epoch {}", provided_epoch[0]);

		let current_epoch_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;

		let key_age = (current_epoch_time as i64) - provided_epoch[0].parse::<i64>().unwrap();
		println!("key_age {}", key_age);

		if key_age > 86400 {
			return AccessKeyHandler {
				valid: false,
				key: existing_key.to_string()
			}
		}
		else {
			return AccessKeyHandler {
				valid: true,
				key: existing_key.to_string()
			}
		}
	}

	//Create new key
	pub fn generate_key() -> AccessKeyHandler {
		let epoch_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
		let v4_uuid = uuid::Uuid::new_v4();
		let plain_text_key = epoch_time.to_string() + "&" + &v4_uuid.to_string();
		let base64_encoded = base64::encode(&plain_text_key);
		return AccessKeyHandler {
			valid: true,
			key: base64_encoded
		}
	}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
