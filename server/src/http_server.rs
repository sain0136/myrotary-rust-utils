use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs;
use std::io;
use rand::Rng;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let mut request_body = String::new();

    stream.read(&mut buffer).expect("Failed to read from stream");
    request_body.push_str(&String::from_utf8_lossy(&buffer));

    println!("Writing to file");
    write_to_file(&request_body).expect("Failed to write to file");
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    std::io::Write::write_all(&mut stream, response.as_bytes()).expect("Failed to write response");

    stream.shutdown(std::net::Shutdown::Both).expect("Failed to shutdown connection");

    println!("Connection closed");
}

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    // Accept connections and spawn a new thread to handle each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn write_to_file(contents: &str) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen::<u32>();

    let dir_path = "/tmp/rust";
    let file_path = format!("{}/output-{}.txt", dir_path , random_number);

    // Create the directory if it doesn't exist
    fs::create_dir_all(dir_path)?;

    // Write the contents to the file
    fs::write(file_path, contents)?;

    Ok(())
}