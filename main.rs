use std::{net::TcpStream, io::Write};

fn main() {
	let x = 800;
	let y = 800;
	let w = 100;
	let h = 100;
	
	let mut stream = TcpStream::connect("151.217.40.82:1234").unwrap();
	let mut buf = Vec::<u8>::with_capacity(w*h*30);

	for x in x..(x+w) {
		for y in y..(y+h) {
			write!(buf, "PX {} {} 00ff00\n", x, y);
		}
	}
	loop {
		stream.write(&*buf).unwrap();
	}
}