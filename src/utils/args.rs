mod data {
	pub const STATIC_ARGS_NO_END: [&'static str; 1] = ["-sort"];
}

#[derive(Debug)]
pub struct ArgsData {
	pub ip: String,
	pub ip_root: String,
	pub ip_number: u32,
	pub cidr: u8,
	pub mask: String,
	pub sort: bool,
}
impl ArgsData {
	fn new() -> ArgsData {
		ArgsData {
			ip: String::new(),
			ip_root: String::new(),
			ip_number: 0,
			cidr: 0,
			mask: String::new(),
			sort: false
		}
	}
}

pub fn get(args: Vec<String>) -> ArgsData {
	let mut to_return = ArgsData::new();
	for index in 1..args.len() {
		if is_ip(&args[index]) {
			to_return.ip = args[index].to_string();
		} else if is_cidr(&args[index]) {
			to_return.cidr = args[index].parse::<u8>().unwrap();
		} else if is_in_static_args_no_end(&args[index]) {
			if args[index] == "-sort" {
				to_return.sort = true;
			}
		}
	}
	to_return.ip_number = get_ip_number(to_return.cidr);
	to_return.mask = generate_full_mask(to_return.cidr);
	let (ip_root, mask32) = get_ip_root(&to_return.ip);
	if mask32 && to_return.cidr != 32 {
		to_return.cidr = 32;
		to_return.mask = String::from("255.255.255.255");
		to_return.ip_root = ip_root;
		to_return.ip_number = 1;
	} else {
		to_return.ip_root = ip_root;
	}
	to_return
}

fn remove_first(s: &str) -> &str {
	&s[1..]
}
fn get_first(s: &str) -> char {
	s.chars().collect::<Vec<char>>()[0]
}

fn is_ip(input: &str) -> bool {
	input.split('.').collect::<Vec<&str>>().len() == 4
}
fn is_cidr(input: &str) -> bool {
	input.parse::<u16>().unwrap_or(999) <= 32
}
fn is_in_static_args_no_end(input: &str) -> bool {
	data::STATIC_ARGS_NO_END.contains(&input)
}


fn get_ip_number(cidr: u8) -> u32 {
	(2 as u32).pow(32 - cidr as u32) - 1
}
fn get_ip_root(ip: &str) -> (String, bool) {
	if ip.chars().last().unwrap() == '0' {
		return (remove_last(ip).to_string(), false);
	}
	println!("Hey !");
	(ip.to_string(), true)
}

pub fn generate_full_mask(cidr_input: u8) -> String {
	let mut cidr = cidr_input.clone();
	let mut mask: String = String::new();
	let mut length_bytes = 0;
	let pow_two = [128,64,32,16,8,4,2,1];
	for _ in 0..4 {
		let mut buffer: u32 = 0;
		for index in 0..8 {
			if cidr == 0 {
				mask.push_str(&format!("{}.", buffer.to_string()));
				length_bytes += 1;
				while length_bytes < 4 {
					mask.push_str("0.");
					length_bytes += 1;
				}
				return remove_last(&mask).to_string();
			}
			cidr -= 1;
			buffer += pow_two[index];
		}
		mask.push_str(&format!("{}.", buffer.to_string()));
		length_bytes += 1;
	}
	remove_last(&mask).to_string()
}
pub fn remove_last(s: &str) -> &str {
	&s[..s.len() - 1]
}