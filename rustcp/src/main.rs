use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // this is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // this line reads data from the stream and stores it in the buffer.
    stream.read(&mut buffer).expect("Failed to read from the client");
    // next we convert the data in the buffer into utf8 encoded string

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received a request: {}", request);
    // this is the response we are sending back to the client
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, Client!".as_bytes();
    // this line writes the response to the stream
    stream.write(response).expect("Failed to write the resposne to the client");
    // this line flushes the stream
    stream.flush().expect("Failed to flush the stream");

}


// Entry point
fn main() {
   let listener = TcpListener::bind("127.0.0.1:8081").
   expect("Failed to bind to address");
   println!("Server Listening on port 127:0.0.1:8081");

   for stream in listener.incoming() {
       match stream {
           Ok(stream) => {
            std::thread::spawn(||  handle_client
                (stream));
           }
           Err(e) => {
               eprintln!("Failed to establish a connection: {}", e);
           // stderr - standard error stream
            }
       }
   }
}