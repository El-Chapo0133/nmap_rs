use pnet::datalink::{self,NetworkInterface};
use pnet::datalink::Channel::Ethernet;

use std::boxed::Box;


fn main() {
	let interface = get_interface("lo".to_string());

	println!("{:?}", interface);

	let (mut sender, mut receiver) = get_channels(interface);
}

fn get_interface(interface_name: String) -> NetworkInterface {
	let interfaces = datalink::interfaces();
	let interface_match = |iface: &NetworkInterface| iface.name == interface_name;

	interfaces.into_iter().filter(interface_match)
		  .next().unwrap()
}

fn get_channels(interface: NetworkInterface) -> (Box<(dyn datalink::DataLinkSender + 'static)>, Box<(dyn datalink::DataLinkReceiver + 'static)>) {
	match datalink::channel(&interface, Default::default()) {
		Ok(Ethernet(tx,rx)) => (tx,rx),
		Ok(_) => panic!("Unheandled channel type"),
		Err(why) => panic!("An error occurred when creating the channels: {}", why)
	}
}