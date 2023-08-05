use std::process::Command;

pub fn list_devices() -> String {
	let res = Command::new("cmd")
		.arg("/C")
		.arg("adb")
		.arg("devices")
		.output()
		.expect("Failed to run command!");
	let err = String::from_utf8_lossy(&res.stderr);
	if err.len() > 0 {
		println!("err: {}", err);
	}
	String::from_utf8_lossy(&res.stdout).to_string()
}

pub fn pair(address: &str, code: &str) {
	let res = Command::new("cmd")
		.arg("/C")
		.arg("adb")
		.arg("pair")
		.arg(address)
		.arg(code)
		.output()
		.expect("Failed to run command!");
	let err = String::from_utf8_lossy(&res.stderr);
	if err.len() > 0 {
		println!("err: {}", err);
	}
}

pub fn connect(address: &str) {
	let res = Command::new("cmd")
		.arg("/C")
		.arg("adb")
		.arg("connect")
		.arg(address)
		.output()
		.expect("Failed to run command!");
	let err = String::from_utf8_lossy(&res.stderr);
	if err.len() > 0 {
		println!("err: {}", err);
	}
}

pub fn disconnect(address: &str) {
	let res = Command::new("cmd")
		.arg("/C")
		.arg("adb")
		.arg("disconnect")
		.arg(address)
		.output()
		.expect("Failed to run command!");
	let err = String::from_utf8_lossy(&res.stderr);
	if err.len() > 0 {
		println!("err: {}", err);
	}
}