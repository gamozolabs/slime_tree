use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:13370")?;

    let mut buffer = vec![0u8; 64 * 1024];

    for stream in listener.incoming() {
        print!("Got new connection\n");

        let mut stream = stream?;

        loop {
            if let Ok(bread) = stream.read(&mut buffer) {
                // Connection closed, break out
                if bread == 0 {
                    break;
                }

                // Send acknowledge
                stream.write(b"ACK").expect("Failed to send ack");
                stream.flush().expect("Failed to flush");
                
                let string = std::str::from_utf8(&buffer[..bread])
                    .expect("Invalid UTF-8 character in string");
                print!("Fuzzing: {}\n", string);
            } else {
                // Failed to read, break out
                break;
            }
        }
    }

    Ok(())
}

