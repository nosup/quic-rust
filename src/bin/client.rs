use std::{io::{self, Write}, net::UdpSocket};


fn main() {
    const SERVER_ADDR: &str = "127.0.0.1:4433";
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    loop {
        print!("Enter message: ");

        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        socket.send_to(input.trim().as_bytes(), SERVER_ADDR).unwrap();

        let mut buf = [0u8; 1024];
        let (amt, _) = socket.recv_from(&mut buf).unwrap();
        println!("Server: {}", String::from_utf8_lossy(&buf[..amt]));
    }
}

// fn main() {
//     println!("Starting UDP client...");

//     // Bind to any available port (like letting OS choose)
//     let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind socket");

//     let server_addr = "127.0.0.1:4433";
//     let message = b"Hello from Rust!"; // b"..." = byte string literal

//     println!("Sending to {}...", server_addr);

//     // Send message
//     socket
//         .send_to(message, server_addr)
//         .expect("Failed to send data");

//     println!("   Sent: {}", String::from_utf8_lossy(message));

//     // Wait for response
//     let mut buf: [u8; 1024] = [0u8; 1024];
//     let (amt, src) = socket
//         .recv_from(&mut buf)
//         .expect("Failed to receive response");

//     println!("Received {} bytes from {}", amt, src);
//     println!("Raw bytes: {:?}", &buf[..amt]);
//     let response = String::from_utf8_lossy(&buf[..amt]);
//     println!("   Response: [{}]", response);
// }
