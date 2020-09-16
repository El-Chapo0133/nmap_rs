mod utils;
mod threads;

use std::sync::mpsc::{channel,Sender,Receiver};

struct IpAddr {
	root: String,
	ip_number: u32, // number of ip to test
}


fn main() {
	println!("-- Use this script for windows cmd");

	let args_data: utils::args::ArgsData = utils::args::get(std::env::args().collect::<Vec<String>>());

	println!("{:?}", args_data);

	let (sender, receiver): (Sender<threads::DataThread>, Receiver<threads::DataThread>) = channel();

	start_all_threads(args_data.ip_root, args_data.ip_number, sender);

	let mut data_received: Vec<threads::DataThread> = Vec::new();
	let iterator = 0;

	for received in receiver {
		data_received.push(received);
		//println!("{0}\n{1}/{2} ({3})\n", received.hostname, received.ip, args_data.cidr, args_data.mask);
		iterator += 1;
		if iterator == args_data.ip_number {
			break;
		}
	}

	if args_data.sort {
		data_received = utils::sort_by_ips(data_received);
	}
}

fn start_all_threads(ip: String, ip_number: u32, sender: std::sync::mpsc::Sender<threads::DataThread>) {
	for index in 1..ip_number {
		// this thing is to exec on another thread
		let ip_command: &str = &format!("{0}{1}", ip, index);
		
		threads::start_thread(ip_command.to_string(), sender.clone());
	}
}