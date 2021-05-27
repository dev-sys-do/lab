use std::fs;
use std::io::Read;
use std::net::TcpListener;

use micro_http::{Body, Response, StatusCode, Version};

#[derive(Debug)]
pub enum PimaError {
    IoError(std::io::Error),
}

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_IP: &str = "0.0.0.0";
const DEFAULT_SERVER: &str = "pima 0.1";
const DEFAULT_404: &str = "/tmp/pima/404.html";

fn main() -> std::result::Result<(), PimaError> {
    let not_found = Body::new(fs::read_to_string(DEFAULT_404).map_err(PimaError::IoError)?);
    let server = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT))
        .map_err(PimaError::IoError)?;

    println!(
        "Waiting for incoming clients on {}:{}",
        DEFAULT_IP, DEFAULT_PORT
    );

    for stream in server.incoming() {
        let mut stream = stream.map_err(PimaError::IoError)?;
        let mut client_data = [0; 1024];

        println!(
            "New client {:?}",
            stream.peer_addr().map_err(PimaError::IoError)?
        );

        let mut response = Response::new(Version::Http11, StatusCode::NotFound);
        response.set_server(DEFAULT_SERVER);
        response.set_body(not_found.clone());

        stream.read(&mut client_data).map_err(PimaError::IoError)?;
        response
            .write_all(&mut stream)
            .map_err(PimaError::IoError)?;
    }

    Ok(())
}
