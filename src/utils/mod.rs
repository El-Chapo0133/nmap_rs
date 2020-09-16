pub mod args;

struct DataThread {
	ping: bool,
	hostname: String,
	ip: String,
}

pub fn sort_by_ips(input: DataThread) -> DataThread {
	/* filter with `sort_unstable_by_key()`
	 * like input_cloned.sort_unstable_by_key(|data| data.ip.split('.').collect::<&str>()[3].parse::<u8>());
	 */
}