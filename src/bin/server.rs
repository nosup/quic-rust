use std::net::UdpSocket;

fn main() {
    println!("Starting UDP server...");

    let socket = UdpSocket::bind("127.0.0.1:4433").expect("Failed to bind socket");

    println!("Listening on 127.0.0.1:4433");

    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        // Receive data (blocking call - waits until data arrives)
        // Returns: (bytes_received, sender_address)

        buf.fill(0); // Caught a bug without this here. Need to clear the buffer with 0s again.

        let (amt, src) = socket.recv_from(&mut buf).expect("Failed to receive data");

        println!("Received {} bytes from {}", amt, src);
        println!("Raw bytes: {:?}", &buf[..amt]);
        println!("Raw first 50 bytes: {:?}", &buf[..50]);

        // Print what we received (as UTF-8 string)
        let received: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf[..amt]);
        println!("   Data: {}", received);

        // Echo it back
        socket
            .send_to(&buf[..amt], src)
            .expect("Failed to send response");

        println!("Echoed back to {}", src);
    }
}
