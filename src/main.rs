use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};

mod virtual_directory;

use virtual_directory::*;

fn handle_connection(mut file: std::fs::File, socket: SocketAddr, mut stream: TcpStream) {
	println!("I'm in start of handle connection");
    let mut reply = Vec::<u8>::new();
    let mut buffer = [0; 1024];

    write!(reply, "{} {}\r\n", 220 as u32, "Welcome to FTP server.");
    match stream.write(&reply) {
        Ok(_) => {
			reply.clear();
		}
        Err(e) => println!("Connection error: {}", e),
    }
	match stream.read(&mut buffer) {
		Ok(length) => {
			//println!("{}", String::from_utf8_lossy(&buffer));
			writeln!(file, "{} {}", &socket, String::from_utf8_lossy(&buffer[0..length]));
		}
		Err(e) => println!("Login input error!"),
	}
	write!(reply, "{} {}\r\n", 331 as u32, "Please specify the password.");
	match stream.write(&reply) {
		Ok(_) => reply.clear(),
		Err(_) => {}
	}
	println!("Send password request.");
	match stream.read(&mut buffer) {
		Ok(length) => {
			//println!("{}", String::from_utf8_lossy(&buffer));
			writeln!(file, "{} {}", &socket, String::from_utf8_lossy(&buffer[0..length]));
		}
		Err(_) => {}
	}
	write!(reply, "{} {}\r\n", 230 as u32, "Login successfull.");
	match stream.write(&reply) {
		Ok(_) => reply.clear(),
		Err(_) => {}
	}
	let mut address = String::from("");
	loop {
		let mut length = 0;
		match stream.read(&mut buffer) {
			Ok(l) => {
				writeln!(file, "{} {}", &socket, String::from_utf8_lossy(&buffer[0..l]));
				length = l;
				for i in length..1024 {
					buffer[i] = 0;
				}

			}
			Err(_) => {}
		}
		let client_request = String::from_utf8_lossy(&buffer[0..length]).replace("\r\n", "");
		println!("Handled request: {}", &client_request);
		let client_request_tokens: Vec<&str> = client_request.split(' ').collect();
		println!("{}", &client_request_tokens[0]);
		if client_request_tokens[0] != "SYST\r\n" {
			println!("Broken shit");
		}
		let bytes = client_request.as_bytes();
		for byte in bytes.iter() {
			println!("{:X}", byte);
		}
		match client_request_tokens[0] {
			"SYST" => {
				println!("Process SYST");
				write!(reply, "{} {}\r\n", 215 as u32, "UNIX Type: L8");
				match stream.write(&reply) {
					Ok(_) => reply.clear(),
					Err(_) => {}
				}
			}
			"PORT" => {
				let a: Vec<i32> = client_request_tokens[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
				address = format!("{}.{}.{}.{}:{}", a[0], a[1], a[2], a[3], a[4] * 256 + a[5]);
				write!(reply, "{} {}\r\n", 200 as u32, "PORT command successfull.");
				match stream.write(&reply) {
					Ok(_) => reply.clear(),
					Err(_) => {}
				}
			}
			"LIST" => {
				{
					let mut passive_stream = TcpStream::connect(&address).unwrap();
					write!(reply, "{} {}\r\n", 150 as u32, "Here comes the directory listing.");
					match stream.write(&reply) {
						Ok(_) => reply.clear(),
						Err(_) => {}
					}
					let	fs_listing = vec!["-rw-rw-r--    1 1000     1000          669 Mar 13 19:59 result-10.txt\r\n",
									  "-rw-rw-r--    1 1000     1000          669 Mar 13 19:59 result-11.txt\r\n",
									  "-rw-rw-r--    1 1000     1000          669 Mar 13 19:59 result-12.txt\r\n"];
					for file in fs_listing.iter() {
						reply.extend_from_slice(file.as_bytes());
					}
					match passive_stream.write(&reply) {
						Ok(_) => reply.clear(),
						Err(_) => {}
					}
				}
				write!(reply, "{} {}\r\n", 226 as u32, "Directory send OK.");
				match stream.write(&reply) {
					Ok(_) => reply.clear(),
					Err(_) => {}
				}
			}
			"RETR" => {
				{
					let mut passive_stream = TcpStream::connect(&address).unwrap();
					write!(reply, "{} {} {}.\r\n", 150 as u32, "Opening binary connection mode for", &client_request_tokens[1]);
					match stream.write(&reply) {
						Ok(_) => reply.clear(),
						Err(_) => {}
					}
					let	fs_listing = vec!["some test data"];
					for file in fs_listing.iter() {
						reply.extend_from_slice(file.as_bytes());
					}
					match passive_stream.write(&reply) {
						Ok(_) => reply.clear(),
						Err(_) => {}
					}
				}
				write!(reply, "{} {}\r\n", 226 as u32, "Transfer complete.");
				match stream.write(&reply) {
					Ok(_) => reply.clear(),
					Err(_) => {}
				}
			}
			"TYPE" => {
				write!(reply, "{} {}\r\n", 200 as u32, "Switching to binary mode.");
				match stream.write(&reply) {
					Ok(_) => reply.clear(),
					Err(_) => {}
				}

			}
			"STOR" => {
				write!(reply, "{} {}\r\n", 550 as u32, "Permission deny.");
				match stream.write(&reply) {
					Ok(_) => reply.clear(),
					Err(_) => {}
				}
			}
			"CWD" => {
				write!(reply, "{} {}\r\n", 250 as u32, "Directory successfully changed.");
				match stream.write(&reply) {
					Ok(_) => reply.clear(),
					Err(_) => {}
				}
			}
			_ => {}
		}
	}
}

fn main() {
    /*let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	let mut file = std::fs::File::create("/home/mikle/Code/logs.txt").unwrap();
	let (stream, socket) = listener.accept().unwrap();
    handle_connection(file, socket, stream);*/
	let user_name = String::from("mikle");
	let file = generate_virtual_directory(&user_name);
	println!("{}", file);
}
	
