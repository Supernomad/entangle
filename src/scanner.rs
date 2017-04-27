use std::net;
use std::thread;

pub fn scan(target: &String, start_port: i32, end_port: i32) {
	for port in start_port..end_port + 1 {
		let host = target.to_owned();
		thread::spawn(move || {
			let open = net::TcpStream::connect((host.as_str(), port as u16)).is_ok();
			if open {
				println!("host {} is listening on port {}.", &host, port)
			}
		});
	}
}
