use std::thread;
use dns_lookup::lookup_addr;

pub struct DataThread {
	pub ping: bool,
	pub hostname: String,
	pub ip: String,
}

pub fn start_thread(ip: String, sender: std::sync::mpsc::Sender<DataThread>) {
	thread::spawn(move || {
		let ping: bool = exec_and_check_command(&format!("ping {} -n 2", &ip));

		if ping {
			sender.send(DataThread { ping: true, hostname: get_hostname(&ip), ip }).expect("Could not send data throught the sender");
		} else {
			sender.send(DataThread { ping: false, hostname: String::new(), ip: String::new() }).expect("Could not send data throught the sender");
		}
	});
}


fn exec_and_check_command(command: &str) -> bool {
	let command_out = std::process::Command::new("cmd").args(&["/C", command]).output().expect("Failed to execute process");
	convert_to_string(command_out.stdout).contains("ponse")
}

fn convert_to_string(input: Vec<u8>) -> String {
	let mut to_return: String = String::new();
	for cell in input.iter() {
		to_return.push(*cell as char);
	}
	to_return
}
fn get_hostname(input: &str) -> String {
	let ip: std::net::IpAddr = input.parse().unwrap();
	return lookup_addr(&ip).unwrap_or(String::from("Not found")).to_string();
}